#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    int fibonacci_1(int a, int b, int n) {
        if (n <= 1) return b;
        return fibonacci_1(b, a + b, n - 1);
    }

    int fibonacci_2(int n) {
        if (n == 2) return 2;
        if (n == 1) return 1;
        return fibonacci_2(n - 1) + fibonacci_2(n - 2);
    }

    int climbStairs(int n) {
        return fibonacci_1(1, 1, n);
    }
};
