#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    vector<vector<string>> groupAnagrams(vector<string>& strs) {
        unordered_map<string, vector<string>> anagrams;

        for (auto& str : strs) {
            string originalStr(str);
            sort(str.begin(), str.end());

            auto iter = anagrams.find(str);
            if (iter == anagrams.end()) {
                anagrams.insert({str, {originalStr}});
            } else {
                iter->second.emplace_back(originalStr);
            }
        }

        vector<vector<string>> result;
        for (auto& [key, value] : anagrams) {
            result.emplace_back(value);
        }

        return result;
    }
};
