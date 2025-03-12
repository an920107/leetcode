#include <bits/stdc++.h>
using namespace std;

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
    bool hasPathSum(TreeNode *root, int targetSum) {
        unordered_map<TreeNode *, bool> visited;

        function<bool(TreeNode *, int)> dfs = [&](TreeNode *root, int currentSum) -> bool {
            if (!root) return false;

            if (visited[root]) return false;
            visited[root] = true;
            currentSum += root->val;

            if (!root->left && !root->right) {
                return currentSum == targetSum;
            }

            return dfs(root->left, currentSum) || dfs(root->right, currentSum);
        };

        return dfs(root, 0);
    }
};
