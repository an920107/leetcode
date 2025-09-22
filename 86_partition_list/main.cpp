struct ListNode {
    int val;
    ListNode* next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode* next) : val(x), next(next) {}
};

class Solution {
   public:
    ListNode* partition(ListNode* head, int x) {
        ListNode* current_node = head;
        ListNode* insert_node = nullptr;
        ListNode* last_node = nullptr;

        while (current_node) {
            if (current_node->val < x) {
                ListNode* tmp_node = current_node;

                if (last_node) {
                    last_node->next = tmp_node->next;
                } else {
                    head = head->next;
                }

                if (insert_node) {
                    tmp_node->next = insert_node->next;
                    insert_node->next = tmp_node;
                    insert_node = tmp_node;
                } else {
                    tmp_node->next = head;
                    head = tmp_node;
                    insert_node = tmp_node;
                }
            }

            last_node = current_node;
            current_node = current_node->next;
        }

        return head;
    }
};
