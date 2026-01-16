#include <cmath>
#include <queue>
#include <unordered_map>
#include <unordered_set>
#include <vector>
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
    TreeNode* subtreeWithAllDeepest(TreeNode* root) {
        // node, parent, depth
        queue<tuple<TreeNode*, TreeNode*, int>> bfs_queue({{root, nullptr, 0}});
        // node, parent
        unordered_map<TreeNode*, TreeNode*> parent_map;
        unordered_set<TreeNode*> nodes;

        int depth = 0;

        while (!bfs_queue.empty()) {
            auto [node, parent, current_depth] = bfs_queue.front();
            bfs_queue.pop();

            if (node == nullptr) {
                continue;
            }

            if (current_depth > depth) {
                nodes.clear();
                depth = current_depth;
            }
            nodes.insert(node);
            parent_map[node] = parent;

            bfs_queue.emplace(node->left, node, current_depth + 1);
            bfs_queue.emplace(node->right, node, current_depth + 1);
        }

        while (nodes.size() > 1) {
            for (auto node : vector<TreeNode*>(nodes.begin(), nodes.end())) {
                nodes.erase(node);
                nodes.insert(parent_map[node]);
            }
        }

        return *nodes.begin();
    }
};
