#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    double myPow(double x, long long n) {
        if (x == 0) return 0;
        if (n == 0) return 1;

        bool isExpNeg = n < 0;
        n = abs(n);

        int logn = (int)log2(n);

        vector<double> expTable{x};

        for (int i = 1; i < logn + 1; i++) {
            expTable.emplace_back(expTable.back() * expTable.back());
        }

        double result = 1.0;
        for (int i = 0; i < logn + 1; i++) {
            if (n & 1) result *= expTable[i];
            n >>= 1;
        }

        if (isExpNeg) result = 1.0 / result;

        return result;
    }
};

/*

 1010
&0001
-----
    0

  101
& 001
-----
    1

x^10

x^8 x^(2^3) 1
x^4 x^(2^2) 0
x^2 x^(2^1) 1
x^1 x^(2^0) 0

*/
