#include <bits/stdc++.h>
using namespace std;

struct ListNode {
    int val;
    ListNode* next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode* next) : val(x), next(next) {}
};

class Solution {
   public:
    ListNode* swapPairs(ListNode* head) {
        ListNode* currentNode = head;
        ListNode* lastNode = nullptr;
        ListNode* lastLastNode = nullptr;

        int count = 0;
        while (currentNode != nullptr) {
            if (count % 2 == 1) {
                ListNode* nextNode = currentNode->next;
                currentNode->next = lastNode;
                lastNode->next = nextNode;

                if (count / 2 == 0) {
                    head = currentNode;
                } else {
                    lastLastNode->next = currentNode;
                }
                
                lastLastNode = currentNode;
                currentNode = nextNode;
            } else {
                lastLastNode = lastNode;
                lastNode = currentNode;
                currentNode = currentNode->next;
            }
            count++;
        }

        return head;
    }
};

int main() {
    ListNode* head = new ListNode(1);
    ListNode* currentNode = head;
    for (int i = 1; i < 4; i++) {
        currentNode->next = new ListNode(i + 1);
        currentNode = currentNode->next;
    }
    Solution().swapPairs(head);
    return 0;
}
