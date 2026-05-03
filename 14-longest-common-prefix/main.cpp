#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    string longestCommonPrefix(vector<string>& strs) {
        if (strs.empty()) {
            return "";
        }

        string result = "";

        int longestLen = 0;
        for (auto &str : strs) {
            longestLen = max(longestLen, (int)str.length());
        }
        
        for (int i = 0; i < longestLen; i++) {
            char currentChar = strs[0][i];
            for (auto &str : strs) {
                if (currentChar != str[i]) {
                    return result;
                }
            }
            result.push_back(currentChar);
        }

        return result;
    }
};
