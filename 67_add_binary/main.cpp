#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    string addBinary(string a, string b) {
        reverse(a.begin(), a.end());
        reverse(b.begin(), b.end());

        string result;
        int carry = 0;

        for (int i = 0; i < a.length() || i < b.length(); i++) {
            int da = i >= a.length() ? 0 : a[i] - '0';
            int db = i >= b.length() ? 0 : b[i] - '0';

            int dc = da + db + carry;
            carry = dc / 2;
            dc %= 2;

            result.push_back(dc + '0');
        }

        if (carry) result.push_back(carry + '0');

        reverse(result.begin(), result.end());

        return result;
    }
};
