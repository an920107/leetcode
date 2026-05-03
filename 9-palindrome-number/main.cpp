#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    bool isPalindrome(int x) {
        if (x < 0) return false;

        string num = to_string(x);
        int len = num.length();
        for (int i = 0; i < len / 2; i++) {
            if (num[i] != num[len - i - 1]) {
                return false;
            }
        }

        return true;
    }
};