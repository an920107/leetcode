struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
  public:
    ListNode *deleteMiddle(ListNode *head) {
        if (head->next == nullptr) {
            return nullptr;
        }

        ListNode *prev = nullptr;
        ListNode *slow = head;
        ListNode *fast = head;

        while (fast != nullptr) {
            fast = fast->next;
            if (fast == nullptr) {
                break;
            }
            prev = slow;
            slow = slow->next;
            fast = fast->next;
        }

        prev->next = slow->next;
        delete slow;

        return head;
    }
};
