#include <bits/stdc++.h>
using namespace std;

struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class Solution {
   public:
    TreeNode *sortedListToBST(ListNode *head) {
        if (!head) return nullptr;

        vector<int> nums;

        ListNode *currentNode = head;
        while (currentNode) {
            nums.emplace_back(currentNode->val);
            currentNode = currentNode->next;
        }

        TreeNode *root = new TreeNode();
        appendArrayToNode(nums, root);
        return root;
    }

    void appendArrayToNode(vector<int> &nums, TreeNode *root) {
        int mid = nums.size() / 2;
        root->val = nums[mid];

        if (nums.size() == 1) return;

        vector<int> leftNums, rightNums;
        copy(nums.begin(), nums.begin() + mid, back_inserter(leftNums));
        copy(nums.begin() + mid + 1, nums.end(), back_inserter(rightNums));

        if (!leftNums.empty()) {
            root->left = new TreeNode();
            appendArrayToNode(leftNums, root->left);
        }

        if (!rightNums.empty()) {
            root->right = new TreeNode();
            appendArrayToNode(rightNums, root->right);
        }
    }
};