#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

class Node {
   public:
    Node* prev;
    Node* next;
    ll val;
    int index;

    Node(Node* prev, Node* next, ll val, int index) : prev(prev), next(next), val(val), index(index) {}
    ~Node() {}
};

class PqParams {
   public:
    ll sum;
    int index;
    Node* node;

    PqParams(ll sum, int index, Node* node) : sum(sum), index(index), node(node) {}
    ~PqParams() {}
};

class Compare {
   public:
    bool operator()(PqParams a, PqParams b) {
        if (a.sum < b.sum) {
            return false;
        } else if (a.sum == b.sum) {
            return a.index > b.index;
        }
        return true;
    }
};

class Solution {
   public:
    int minimumPairRemoval(vector<int>& nums) {
        unordered_set<Node*> descent_nodes;
        priority_queue<PqParams, vector<PqParams>, Compare> sum_min_heap;
        unordered_set<Node*> to_delete;

        Node* head = new Node(nullptr, nullptr, nums[0], 0);
        Node* current = head;

        for (int i = 1; i < nums.size(); i++) {
            ll sum = nums[i - 1] + nums[i];
            Node* new_node = new Node(current, nullptr, nums[i], i);
            current->next = new_node;
            new_node->prev = current;
            if (nums[i - 1] > nums[i]) {
                descent_nodes.insert(current);
            }
            sum_min_heap.emplace(sum, i - 1, current);
            current = current->next;
        }

        int result = 0;

        while (!descent_nodes.empty()) {
            auto [sum, index, node] = sum_min_heap.top();
            sum_min_heap.pop();

            if (to_delete.find(node) != to_delete.end()) {
                continue;
            }

            if (node->next == nullptr || sum != node->val + node->next->val) {
                continue;
            }

            descent_nodes.erase(node);
            descent_nodes.erase(node->next);
            descent_nodes.erase(node->prev);

            Node* next = node->next;
            to_delete.insert(next);
            if (next != nullptr) {
                node->next = next->next;
                if (node->next != nullptr) {
                    node->next->prev = node;
                }
                node->val = sum;
            }
            if (node->next != nullptr) {
                ll new_sum = node->val + node->next->val;
                sum_min_heap.emplace(new_sum, index, node);
            }
            if (node->prev != nullptr) {
                ll new_sum = node->prev->val + node->val;
                sum_min_heap.emplace(new_sum, node->prev->index, node->prev);
            }

            if (node->next != nullptr && node->val > node->next->val) {
                descent_nodes.insert(node);
            }
            if (node->prev != nullptr && node->prev->val > node->val) {
                descent_nodes.insert(node->prev);
            }
            result++;
        }

        return result;
    }
};

int main(int argc, char const* argv[]) {
    Solution s;
    vector<int> v{-2, 1, 2, -1, -1, -2, -2, -1, -1, 1, 1};
    s.minimumPairRemoval(v);
}
