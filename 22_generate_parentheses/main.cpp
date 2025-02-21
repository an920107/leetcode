#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    vector<string> generateParenthesis(int n) {
        vector<string> result;

        function<void(string, char, int, int)> recursive = [&](string s, char next, int leftCount, int totalLeftCount) {
            s.push_back(next);

            if (s.length() == n * 2) {
                result.emplace_back(s);
                return;
            }

            if (totalLeftCount < n) {
                recursive(s, '(', leftCount + 1, totalLeftCount + 1);
            }
            if (leftCount > 0) {
                recursive(s, ')', leftCount - 1, totalLeftCount);
            }

        };

        recursive("", '(', 1, 1);
        return result;
    }
};
