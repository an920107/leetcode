#include <unordered_set>
#include <vector>
#include <cmath>
using namespace std;

struct TreeNode
{
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class Solution {
private:
    const int MOD = 1e9 + 7;
    vector<int> sub_sums;

    int dfs(TreeNode *root) {
        if (root == nullptr) {
            return 0;
        }

        int left_sum = dfs(root->left);
        int right_sum = dfs(root->right);

        int current_sum = root->val + left_sum + right_sum;
        sub_sums.emplace_back(current_sum);
        return current_sum;
    }

public:
    int maxProduct(TreeNode *root) {
        int sum = dfs(root);

        int result = 0;
        int min_diff = INT32_MAX;

        for (long long sub_sum_x : sub_sums) {
            long long sub_sum_y = sum - sub_sum_x;
            int diff = abs(sub_sum_x - sub_sum_y);
            if (diff < min_diff) {
                min_diff = diff;
                result = (sub_sum_x * sub_sum_y) % MOD;
            }
        }

        return result;
    }
};