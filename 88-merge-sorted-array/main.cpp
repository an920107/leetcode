#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    void merge(vector<int>& nums3, int m, vector<int>& nums2, int n) {
        vector<int> nums1(nums3);

        for (int i1 = 0, i2 = 0; i1 + i2 < m + n;) {
            int x1 = i1 < m ? nums1[i1] : INT32_MAX;
            int x2 = i2 < n ? nums2[i2] : INT32_MAX;

            if (x1 <= x2) {
                nums3[(i1++) + i2] = x1;
            } else {
                nums3[i1 + (i2++)] = x2;
            }
        }
    }
};
