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
    vector<int> preorderTraversal(TreeNode *root) {
        vector<int> result;
        unordered_map<TreeNode *, bool> visited;

        function<void(TreeNode *)> dfs = [&](TreeNode *root) {
            if (!root) return;
            if (visited[root]) return;
            visited[root] = true;

            result.emplace_back(root->val);
            dfs(root->left);
            dfs(root->right);
        };

        dfs(root);

        return result;
    }
};
