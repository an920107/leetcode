#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    bool isPalindrome(string s) {
        string proceededStr;

        for (auto ch : s) {
            if ((ch >= 'a' && ch <= 'z') || (ch >= '0' && ch <= '9')) {
                proceededStr.push_back(ch);
            } else if (ch >= 'A' && ch <= 'Z') {
                proceededStr.push_back(ch - ('A' - 'a'));
            }
        }

        cout << proceededStr << "\n";

        const int len = proceededStr.length();
        for (int i = 0; i < len / 2; i++) {
            if (proceededStr[i] != proceededStr[len - i - 1]) {
                return false;
            }
        }

        return true;
    }
};
