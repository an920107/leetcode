#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    int findPalindromeLength(const string &str, int center, int skip) {
        for (int radius = skip; center - radius >= 0 && center + radius < str.length(); radius++) {
            char r = str[center + radius];
            char l = str[center - radius];

            if (r != l) {
                return radius - 1;
            }
        }

        return min(center, (int)str.length() - 1 - center);
    }

    string longestPalindrome(string s) {
        string str = "#";
        for (auto c : s) {
            str.push_back(c);
            str.push_back('#');
        }

        vector<int> p(str.length(), 0);
        int center = 0, right = 0, maxRadius = 0;
        for (int index = 1; index < str.length() - 1; index++) {
            if (index > right) {
                int radius = findPalindromeLength(str, index, 1);
                p[index] = radius;
                if (radius >= maxRadius) {
                    maxRadius = radius;
                    center = index;
                    right = center + radius;
                }
                continue;
            }

            int indexMirror = center - (index - center);
            if (index + p[indexMirror] < right) {
                p[index] = p[indexMirror];
                continue;
            }

            int radius = findPalindromeLength(str, index, right - index);
            p[index] = radius;
            if (radius >= maxRadius) {
                maxRadius = radius;
                center = index;
                right = center + radius;
            }
        }

        int start = (center - maxRadius) / 2;
        int length = maxRadius;
        return s.substr(start, length);
    }
};
