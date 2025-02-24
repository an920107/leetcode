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
    ListNode* deleteDuplicates(ListNode* head) {
        ListNode* currentNode = head;
        ListNode* lastNode = nullptr;

        while (currentNode != nullptr) {
            if (lastNode && lastNode->val == currentNode->val) {
                lastNode->next = currentNode->next;
                delete currentNode;
                currentNode = lastNode->next;
            } else {
                lastNode = currentNode;
                currentNode = currentNode->next;
            }
        }

        return head;
    }
};