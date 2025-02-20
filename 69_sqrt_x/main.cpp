#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    int mySqrt(int x) {
        if (x == 1) return 1;

        long long l = 0, r = x;

        while (true) {
            long long m = (l + r) / 2;
            if (m * m > x) {
                r = m;
            } else if (m * m < x) {
                if ((m + 1) * (m + 1) > x) {
                    return m;
                }
                l = m;
            } else {
                return m;
            }
        }

        return 0;
    }
};
