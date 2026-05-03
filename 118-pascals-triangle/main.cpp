#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    vector<vector<int>> generate(int numRows) {
        vector<vector<int>> result{{1}};
        for (int i = 1; i < numRows; i++) {
            auto lastRow = result[i - 1];

            vector<int> currentRow(i + 1, 0);
            currentRow[0] = 1;
            currentRow[i] = 1;

            for (int j = 1; j < i; j++) {
                currentRow[j] = lastRow[j - 1] + lastRow[j];
            }

            result.emplace_back(currentRow);
        }
        return result;
    }
};
