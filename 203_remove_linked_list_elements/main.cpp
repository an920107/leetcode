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
    ListNode* removeElements(ListNode* head, int target) {
        ListNode* tempHead = new ListNode(-1, head);
        ListNode* currentNode = tempHead;
        ListNode* lastNode = nullptr;

        while (currentNode) {
            if (currentNode->val == target) {
                lastNode->next = currentNode->next;
                currentNode = currentNode->next;
            } else {
                lastNode = currentNode;
                currentNode = currentNode->next;
            }
        }

        return tempHead->next;
    }
};
