#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    vector<int> getRow(int rowIndex) {
        vector<int> lastRow{1};
        for (int i = 1; i < rowIndex + 1; i++) {
            vector<int> currentRow(i + 1, 0);
            currentRow[0] = 1;
            currentRow[i] = 1;

            for (int j = 1; j < i; j++) {
                currentRow[j] = lastRow[j - 1] + lastRow[j];
            }

            lastRow = currentRow;
        }
        return lastRow;
    }
};
