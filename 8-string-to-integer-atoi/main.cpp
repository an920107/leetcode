#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    void removeWhitespace(string &s) {
        int i = 0;
        while (i < s.length() && s[i] == ' ') i++;
        s = s.substr(i);
    }

    void conversion(string &s) {
        int i = 0;
        while (i < s.length() && s[i] == '0') i++;
        s = s.substr(i);

        i = 0;
        while (i < s.length() && s[i] >= '0' && s[i] <= '9') i++;
        s = s.substr(0, i);
    }

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

    int myAtoi(string s) {
        // step 1: ignore whitespace
        removeWhitespace(s);
        if (s.length() == 0) return 0;

        // step 2: signedness
        bool isNeg = false;
        if (s[0] == '+') {
            s = s.substr(1);
        } else if (s[0] == '-') {
            s = s.substr(1);
            isNeg = true;
        }
        if (s.length() == 0) return 0;

        // step 3: conversion
        conversion(s);
        if (s.length() == 0) return 0;

        // step 4: rounding
        if (isNeg) s.insert(s.begin(), '-');
        if (isLessThan(s, to_string(INT32_MIN))) return INT32_MIN;
        if (isLessThan(to_string(INT32_MAX), s)) return INT32_MAX;
        return atoi(s.c_str());
    }
};