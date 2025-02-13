#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    string convert(string s, int rows) {
        if (rows == 1) {
            return s;
        }

        string result;
        int tolerance = rows * 2 - 2;

        for (int i = 0; i < s.length(); i += tolerance) {
            result.push_back(s[i]);
        }

        for (int j = 1; j <= rows - 2; j++) {
            for (int r = 0; r <= s.length() / tolerance; r++) {
                int i = r * tolerance + j;
                if (i < s.length()) {
                    result.push_back(s[i]);
                }
                i = (r + 1) * tolerance - j;
                if (i < s.length()) {
                    result.push_back(s[i]);
                }
            }
        }

        for (int i = rows - 1; i < s.length(); i += tolerance) {
            result.push_back(s[i]);
        }

        return result;
    }
};
