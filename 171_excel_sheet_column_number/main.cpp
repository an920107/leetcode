#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    int titleToNumber(string columnTitle) {
        reverse(columnTitle.begin(), columnTitle.end());

        int result = 0;

        for (int i = 0; i < columnTitle.length(); i++) {
            const char c = columnTitle[i];
            result += (c - 'A' + 1) * (int)pow(26, i);
        }

        return result;
    }
};
