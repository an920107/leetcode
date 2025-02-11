#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    int romanToInt(string s) {
        int result = 0;

        for (int i = 0; i < s.length(); i++) {
            switch (s[i]) {
                case 'M':
                    result += 1000;
                    break;
                case 'D':
                    result += 500;
                    break;
                case 'C':
                    if (i < s.length() - 1 && (s[i + 1] == 'M' || s[i + 1] == 'D')) {
                        result -= 100;
                    } else {
                        result += 100;
                    }
                    break;
                case 'L':
                    result += 50;
                    break;
                case 'X':
                    if (i < s.length() - 1 && (s[i + 1] == 'C' || s[i + 1] == 'L')) {
                        result -= 10;
                    } else {
                        result += 10;
                    }
                    break;
                case 'V':
                    result += 5;
                    break;
                case 'I':
                    if (i < s.length() - 1 && (s[i + 1] == 'X' || s[i + 1] == 'V')) {
                        result -= 1;
                    } else {
                        result += 1;
                    }
                    break;
            }
        }

        return result;
    }
};