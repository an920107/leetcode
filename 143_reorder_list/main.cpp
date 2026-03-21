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
   private:
    ListNode* split_off_mid(ListNode* head) {
        ListNode* last1 = nullptr;
        ListNode* current1 = head;
        ListNode* current2 = head;

        while (current1 && current2) {
            last1 = current1;
            current1 = current1->next;
            current2 = current2->next;

            if (current2 == nullptr) {
                break;
            }
            current2 = current2->next;
        }

        last1->next = nullptr;
        return current1;
    }

    ListNode* reverse_list(ListNode* head) {
        ListNode* current = head;
        ListNode* last = nullptr;
        while (current != nullptr) {
            ListNode* next = current->next;
            current->next = last;
            last = current;
            current = next;
        }
        return last;
    }

   public:
    void reorderList(ListNode* head) {
        ListNode* mid_node = split_off_mid(head);
        ListNode* left = head;
        ListNode* reverse_right = reverse_list(mid_node);

        ListNode* current1 = left;
        ListNode* current2 = reverse_right;
        while (true) {
            ListNode* next1 = current1->next;
            current1->next = current2;
            current1 = next1;

            if (!current1) {
                break;
            }

            ListNode* next2 = current2->next;
            current2->next = current1;
            current2 = next2;

            if (!current2) {
                break;
            }
        }
    }
};