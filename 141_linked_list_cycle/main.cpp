#include <bits/stdc++.h>
using namespace std;

struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
   public:
    bool hasCycle(ListNode *head) {
        if (!head) return false;

        ListNode *slowNode = head;
        ListNode *fastNode = head->next;

        while (fastNode && fastNode->next) {
            if (slowNode == fastNode && fastNode) {
                return true;
            }

            slowNode = slowNode->next;
            fastNode = fastNode->next->next;
        }

        return false;
    }
};

int main() {
    ListNode *head = new ListNode(1);
    head->next = new ListNode(2);

    Solution().hasCycle(head);

    return 0;
}
