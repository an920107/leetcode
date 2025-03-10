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
    int minDepth(TreeNode *root) {
        queue<TreeNode *> bfsQueue;
        unordered_map<TreeNode *, int> depthTable;

        if (root) {
            depthTable[root] = 1;
            bfsQueue.emplace(root);
        }

        while (!bfsQueue.empty()) {
            TreeNode *currentNode = bfsQueue.front();
            bfsQueue.pop();

            if (!currentNode->left && !currentNode->right) {
                return depthTable[currentNode];
            }

            if (currentNode->left) {
                depthTable[currentNode->left] = depthTable[currentNode] + 1;
                bfsQueue.emplace(currentNode->left);
            }
            if (currentNode->right) {
                depthTable[currentNode->right] = depthTable[currentNode] + 1;
                bfsQueue.emplace(currentNode->right);
            }
        }

        return 0;
    }
};
