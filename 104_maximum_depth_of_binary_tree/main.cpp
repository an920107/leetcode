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
    int maxDepth(TreeNode *root) {
        unordered_map<TreeNode *, bool> visited;
        int maxDepth = 0;

        function<void(TreeNode *, int)> dfs = [&](TreeNode *root, int currentDepth) {
            if (!root) return;

            if (visited[root]) return;
            visited[root] = true;

            maxDepth = max(maxDepth, currentDepth);

            dfs(root->left, currentDepth + 1);
            dfs(root->right, currentDepth + 1);
        };

        dfs(root, 1);

        return maxDepth;
    }
};