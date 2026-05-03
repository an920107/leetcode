#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    unordered_map<char, unordered_set<string>> charmap = {
        {'2', {"a", "b", "c"}},
        {'3', {"d", "e", "f"}},
        {'4', {"g", "h", "i"}},
        {'5', {"j", "k", "l"}},
        {'6', {"m", "n", "o"}},
        {'7', {"p", "q", "r", "s"}},
        {'8', {"t", "u", "v"}},
        {'9', {"w", "x", "y", "z"}},
    };

    unordered_set<string> combineTwo(const unordered_set<string> &a, const unordered_set<string> &b) {
        unordered_set<string> result;

        for (auto first : a) {
            for (auto second : b) {
                result.insert(first + second);
            }
        }

        return result;
    }

    vector<string> letterCombinations(string digits) {
        if (digits.length() == 0) return {};

        unordered_set<string> resultSet = charmap[digits[0]];

        for (int i = 1; i < digits.length(); i++) {
            resultSet = combineTwo(resultSet, charmap[digits[i]]);
        }

        vector<string> result;
        for (auto s : resultSet) result.emplace_back(s);

        return result;
    }
};
