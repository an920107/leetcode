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
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        vector<ListNode*> pointerTable;
        int sz = 0;

        ListNode* currentNode = head;
        while (currentNode != nullptr) {
            sz++;
            pointerTable.push_back(currentNode);
            currentNode = currentNode->next;
        }

        if (sz == 1) return nullptr;
        if (sz - n - 1 < 0) return head->next;

        currentNode = pointerTable[pointerTable.size() - n - 1];
        ListNode* target = currentNode->next;
        currentNode->next = currentNode->next->next;
        delete target;

        return head;
    }
};
