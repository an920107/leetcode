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
    ListNode* indexOf(ListNode* head, int index) {
        ListNode* currentNode = head;
        int currentIndex = 0;
        while (currentNode) {
            if (currentIndex == index) return currentNode;
            currentNode = currentNode->next;
            currentIndex++;
        }
        return nullptr;
    }

   public:
    ListNode* rotateRight(ListNode* head, int rotateCount) {
        if (!head) return nullptr;

        ListNode* currentNode = head;
        ListNode* tail = nullptr;
        int len = 0;
        while (currentNode) {
            len++;
            tail = currentNode;
            currentNode = currentNode->next;
        }

        if (len == 1) return head;

        rotateCount %= len;

        if (rotateCount == 0) return head;

        ListNode* newTail = indexOf(head, len - rotateCount - 1);
        ListNode* newHead = newTail->next;
        newTail->next = nullptr;
        tail->next = head;

        return newHead;
    }
};

int main() {
    ListNode* head = new ListNode(1, new ListNode(2, new ListNode(3, new ListNode(4, new ListNode(5)))));
    Solution().rotateRight(head, 2);
    return 0;
}
