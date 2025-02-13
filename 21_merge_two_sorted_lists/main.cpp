#include <bits/stdc++.h>
using namespace std;

struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
   public:
    ListNode *mergeTwoLists(ListNode *list1, ListNode *list2) {
        if (!list1 && !list2) {
            return nullptr;
        }

        ListNode *result = new ListNode{0, nullptr};
        ListNode *currentNode = result;
        ListNode *lastNode = nullptr;

        ListNode *current1 = list1;
        ListNode *current2 = list2;

        while (current1 != nullptr || current2 != nullptr) {
            int val1 = current1 != nullptr ? current1->val : INT32_MAX;
            int val2 = current2 != nullptr ? current2->val : INT32_MAX;

            if (val1 <= val2) {
                currentNode->val = val1;
                current1 = current1->next;
            } else {
                currentNode->val = val2;
                current2 = current2->next;
            }

            currentNode->next = new ListNode{0, nullptr};
            lastNode = currentNode;
            currentNode = currentNode->next;
        }

        lastNode->next = nullptr;
        delete currentNode;

        return result;
    }
};
