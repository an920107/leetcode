#include <bits/stdc++.h>

struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
   public:
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
        std::unordered_set<ListNode *> visited_node;

        ListNode *current = headA;
        while (current) {
            visited_node.insert(current);
            current = current->next;
        }

        current = headB;
        while (current) {
            if (visited_node.find(current) != visited_node.end()) {
                return current;
            }
            current = current->next;
        }

        return NULL;
    }
};