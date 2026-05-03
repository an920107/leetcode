#include <bits/stdc++.h>
using namespace std;

class Solution {
   private:
    // {row, col}
    const vector<pair<int, int>> directions{
        {0, 1},
        {1, 0},
        {0, -1},
        {-1, 0},
    };

    inline pair<int, int> addPair(const pair<int, int>& x, const pair<int, int>& y) {
        return {x.first + y.first, x.second + y.second};
    }

    void nextDirection(int& index, pair<int, int>& direction) {
        if (++index >= 4) index = 0;
        direction = directions[index];
    }

   public:
    vector<vector<int>> generateMatrix(int n) {
        vector<vector<int>> matrix(n + 2, vector<int>(n + 2, 0));
        for (int i = 0; i < n + 2; i++)
            matrix[i][0] = INT32_MAX,
            matrix[i][n + 1] = INT32_MAX,
            matrix[0][i] = INT32_MAX,
            matrix[n + 1][i] = INT32_MAX;

        int directionIndex = 0;
        pair<int, int> direction = directions[directionIndex];
        pair<int, int> currentPos{1, 1};

        for (int i = 1; i <= n * n; i++) {
            auto nextPos = addPair(currentPos, direction);
            if (matrix[nextPos.first][nextPos.second] > 0) {
                nextDirection(directionIndex, direction);
                nextPos = addPair(currentPos, direction);
            }
            matrix[currentPos.first][currentPos.second] = i;
            currentPos = nextPos;
        }

        vector<vector<int>> result(n, vector<int>(n, 0));
        for (int i = 1; i < n + 1; i++)
            for (int j = 1; j < n + 1; j++)
                result[i - 1][j - 1] = matrix[i][j];

        return result;
    }
};

int main() {
    Solution().generateMatrix(3);
    return 0;
}
