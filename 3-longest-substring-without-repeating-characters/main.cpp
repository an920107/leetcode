#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        int maxLen = 0, currentLen = 0;
        queue<int> substr;
        unordered_set<int> charset;

        for (int index = 0; index < s.length(); index++) {
            char currentChar = s[index];

            // has repeat
            if (charset.find(currentChar) != charset.end()) {
                while (substr.front() != currentChar) {
                    charset.erase(substr.front());
                    substr.pop();
                    currentLen--;
                }
                charset.erase(currentChar);
                substr.pop();
                currentLen--;
            }

            charset.emplace(currentChar);
            substr.emplace(currentChar);
            currentLen++;

            maxLen = max(maxLen, currentLen);
        }

        return maxLen;
    }
};
