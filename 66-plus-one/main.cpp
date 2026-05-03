#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    vector<int> plusOne(vector<int>& digits) {
        int carry = 0;
        reverse(digits.begin(), digits.end());
        digits[0]++;
        for (int i = 0; i < digits.size(); i++) {
            digits[i] += carry;
            carry = digits[i] / 10;
            digits[i] %= 10;
        }
        if (carry > 0) digits.emplace_back(carry);
        reverse(digits.begin(), digits.end());

        return digits;
    }
};
