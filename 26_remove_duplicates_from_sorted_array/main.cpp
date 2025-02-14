#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    int removeDuplicates(vector<int>& nums) {
        int k = 0, lastNum = nums[0];

        for (int i = 1; i < nums.size(); i++) {
            if (nums[i] == lastNum) {
                nums[i] = INT32_MAX;
            } else {
                lastNum = nums[i];
            }
        }

        for (int i = 0; i < nums.size(); i++) {
            if (nums[i] != INT32_MAX) {
                nums[k++] = nums[i];
            }
        }

        return k;
    }
};
