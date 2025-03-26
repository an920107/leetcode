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
    vector<int> spiralOrder(vector<vector<int>>& matrix) {
        const int m = matrix.size();
        const int n = matrix[0].size();

        vector<int> result;

        vector<vector<bool>> visited(m + 2, vector<bool>(n + 2, false));
        for (int i = 0; i < m + 2; i++)
            visited[i][0] = true, visited[i][n + 1] = true;
        for (int i = 0; i < n + 2; i++)
            visited[0][i] = true, visited[m + 1][i] = true;

        int directionIndex = 0;
        pair<int, int> direction = directions[directionIndex];

        pair<int, int> currentPos{1, 1};

        for (int i = 0; i < m * n; i++) {
            auto nextPos = addPair(currentPos, direction);
            if (visited[nextPos.first][nextPos.second]) {
                nextDirection(directionIndex, direction);
                nextPos = addPair(currentPos, direction);
            }
            visited[currentPos.first][currentPos.second] = true;
            result.emplace_back(matrix[currentPos.first - 1][currentPos.second - 1]);
            currentPos = nextPos;
        }

        return result;
    }
};
