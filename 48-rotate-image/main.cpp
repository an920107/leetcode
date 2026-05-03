#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    void rotate(vector<vector<int>>& matrix) {
        const int n = matrix.size();
        for (int i = 0; i < n / 2; i++) {
            for (int j = 0; j < n - i * 2 - 1; j++) {
                // matrix[i][i + j]
                // matrix[i + j][n - i - 1]
                // matrix[n - i - 1][n - i - j - 1]
                // matrix[n - i - j - 1][i]
                const int tmp = matrix[n - i - j - 1][i];
                matrix[n - i - j - 1][i] = matrix[n - i - 1][n - i - j - 1];
                matrix[n - i - 1][n - i - j - 1] = matrix[i + j][n - i - 1];
                matrix[i + j][n - i - 1] = matrix[i][i + j];
                matrix[i][i + j] = tmp;
            }
        }
    }
};
