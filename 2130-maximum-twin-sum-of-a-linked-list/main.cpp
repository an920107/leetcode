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
    int pairSum(ListNode *head) {
        int half_n = 0;
        ListNode *current1 = head;
        ListNode *current2 = head;

        while (current2 != nullptr) {
            current1 = current1->next;
            current2 = current2->next->next;
            half_n += 1;
        }

        ListNode *current = current1;
        ListNode *next = current->next;
        current->next = nullptr;
        while (next != nullptr) {
            ListNode *tmp_next = next->next;
            next->next = current;
            current = next;
            next = tmp_next;
        }

        ListNode *front_current = head;
        ListNode *back_current = current;
        int result = 0;
        for (int i = 0; i < half_n; i++) {
            result = max(result, front_current->val + back_current->val);
            front_current = front_current->next;
            back_current = back_current->next;
        }

        return result;
    }
};
