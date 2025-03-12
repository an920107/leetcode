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
    vector<vector<int>> pathSum(TreeNode *root, int targetSum) {
        vector<vector<int>> result;
        unordered_map<TreeNode *, bool> visited;

        function<void(TreeNode *, int, vector<int>)> dfs = [&](TreeNode *root, int currentSum, vector<int> path) {
            if (!root) return;

            if (visited[root]) return;
            visited[root] = true;
            path.emplace_back(root->val);
            currentSum += root->val;

            if (!root->left && !root->right) {
                if (currentSum == targetSum) {
                    result.emplace_back(path);
                }
            }

            dfs(root->left, currentSum, path);
            dfs(root->right, currentSum, path);
        };

        dfs(root, 0, {});

        return result;
    }
};
