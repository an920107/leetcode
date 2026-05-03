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
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        ListNode* result = new ListNode{0, nullptr};
        ListNode* lastNode = nullptr;
        ListNode* currentNode = result;
        int carry = 0;

        while (l1 != nullptr || l2 != nullptr) {
            int n1 = 0, n2 = 0;

            if (l1 != nullptr) {
                n1 = l1->val;
                l1 = l1->next;
            }

            if (l2 != nullptr) {
                n2 = l2->val;
                l2 = l2->next;
            }

            int s = n1 + n2 + carry;
            carry = s / 10;
            s = s % 10;

            currentNode->val = s;
            currentNode->next = new ListNode{0, nullptr};
            lastNode = currentNode;
            currentNode = currentNode->next;
        }

        if (carry > 0) {
            currentNode->val = carry;
        } else if (lastNode != nullptr) {
            delete lastNode->next;
            lastNode->next = nullptr;
        }

        return result;
    }
};
