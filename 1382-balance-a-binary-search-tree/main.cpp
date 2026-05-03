#include <bits/stdc++.h>
using namespace std;

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode* left, TreeNode* right) : val(x), left(left), right(right) {}
};

class Solution {
   private:
    void dfs(TreeNode* root, vector<TreeNode*>& nodes) {
        if (!root) {
            return;
        }

        dfs(root->left, nodes);
        nodes.push_back(root);
        dfs(root->right, nodes);
    }

    TreeNode* build_tree(int left, int right, vector<TreeNode*>& nodes) {
        if (left == right) {
            return nullptr;
        }

        int mid = (left + right) / 2;

        TreeNode* root = nodes[mid];
        root->left = build_tree(left, mid, nodes);
        root->right = build_tree(mid + 1, right, nodes);

        return root;
    }

   public:
    TreeNode* balanceBST(TreeNode* root) {
        vector<TreeNode*> nodes;
        dfs(root, nodes);
        return build_tree(0, nodes.size(), nodes);
    }
};
