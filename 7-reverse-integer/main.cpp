#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    bool isLessThan(string a, string b) {
        bool aIsNeg = false, bIsNeg = false;

        if (a[0] == '-') {
            aIsNeg = true;
            a = a.substr(1);
        }
        if (b[0] == '-') {
            bIsNeg = true;
            b = b.substr(1);
        }

        bool aIsPos = !aIsNeg, bIsPos = !bIsNeg;

        if (aIsNeg && bIsPos) return true;
        if (aIsPos && bIsNeg) return false;

        if (a.length() < b.length()) return aIsPos;
        if (a.length() > b.length()) return aIsNeg;

        for (int i = 0; i < a.length(); i++) {
            if (a[i] < b[i]) return aIsPos;
            if (a[i] > b[i]) return aIsNeg;
        }

        return false;
    }

    int reverse(int x) {
        bool isNeg = x < 0;

        string s = to_string(x);
        if (isNeg) s = s.substr(1);
        std::reverse(s.begin(), s.end());

        if (isNeg) {
            s = "-" + s;
            if (isLessThan(s, to_string(INT32_MIN))) s = "0";
        } else {
            if (isLessThan(to_string(INT32_MAX), s)) s = "0";
        }

        return atoi(s.c_str());
    }
};
