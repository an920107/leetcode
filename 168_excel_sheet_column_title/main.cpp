#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    string convertToTitle(int columnNumber) {
        string result;

        columnNumber--;
        while (columnNumber / 26) {
            int dig = columnNumber % 26;
            result.push_back(dig + 'A');
            columnNumber /= 26;
            columnNumber--;
        }
        int dig = columnNumber % 26;
        result.push_back(dig + 'A');

        reverse(result.begin(), result.end());

        return result;
    }
};
