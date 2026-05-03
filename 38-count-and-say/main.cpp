#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    string rle(string s) {
        s.push_back('.');

        string result;
        int sameCount = 1;

        for (int i = 1; i < s.length(); i++) {
            if (s[i] == s[i - 1]) {
                sameCount++;
            } else {
                result.append(to_string(sameCount) + to_string(s[i - 1] - '0'));
                sameCount = 1;
            }
        }

        return result;
    }

    string countAndSay(int n) {
        string result = "1";
        for (int i = 1; i < n; i++) {
            result = rle(result);
        }
        return result;
    }
};
