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
   public:
    bool isSameTree(TreeNode* p, TreeNode* q) {
        function<bool(TreeNode*, TreeNode*)> dfs = [&](TreeNode* p, TreeNode* q) -> bool {
            // both p and q are null
            if (!p && !q) return true;

            // one of p and q is null
            if (!p || !q) return false;

            // the value of p and q are different
            if (p->val != q->val) return false;

            return dfs(p->left, q->left) && dfs(p->right, q->right);
        };

        return dfs(p, q);
    }
};
