#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    int lengthOfLastWord(string s) {
        int i = s.length() - 1, result = 0;

        while (s[i] == ' ') i--;
        for (; i >= 0 && s[i] != ' '; i--, result++);

        return result;
    }
};
