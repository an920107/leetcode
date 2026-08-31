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
    vector<int> nodesBetweenCriticalPoints(ListNode *head) {
        ListNode *curr_node = head->next;
        ListNode *prev_node = head;
        int index = 1;
        int first_critical = -1;
        int last_critical = -1;
        int curr_critical = -1;
        int min_distance = INT_MAX;

        while (curr_node->next != nullptr) {
            ListNode *next_node = curr_node->next;
            bool is_critical = (prev_node->val < curr_node->val &&
                                next_node->val < curr_node->val) ||
                               (prev_node->val > curr_node->val &&
                                next_node->val > curr_node->val);

            if (is_critical) {
                last_critical = curr_critical;
                curr_critical = index;

                if (first_critical < 0) {
                    first_critical = index;
                }

                if (last_critical > 0) {
                    min_distance =
                        min(min_distance, curr_critical - last_critical);
                }
            }

            prev_node = curr_node;
            curr_node = curr_node->next;
            index += 1;
        }

        if (first_critical == curr_critical) {
            return {-1, -1};
        }

        int max_distance = curr_critical - first_critical;

        return {min_distance, max_distance};
    }
};
