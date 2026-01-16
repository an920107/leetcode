#include <stdlib.h>
#include <string.h>

struct TreeNode {
    int val;
    struct TreeNode* left;
    struct TreeNode* right;
};

int dfs(struct TreeNode* root, int level, int* sum) {
    if (root == NULL) {
        return level - 1;
    }

    sum[level] += root->val;

    int left_depth = dfs(root->left, level + 1, sum);
    int right_depth = dfs(root->right, level + 1, sum);

    return left_depth > right_depth ? left_depth : right_depth;
}

int maxLevelSum(struct TreeNode* root) {
    int* sum = (int*)malloc(sizeof(int) * 10010);
    memset(sum, 0, sizeof(int) * 10010);
    int depth = dfs(root, 0, sum);
    int max_sum = root->val;
    int result = 1;
    for (int i = 0; i <= depth; i++) {
        if (max_sum < sum[i]) {
            max_sum = sum[i];
            result = i + 1;
        }
    }
    free(sum);
    return result;
}
