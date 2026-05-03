#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    int removeElement(vector<int>& nums, int val) {
        if (nums.size() == 0) {
            return 0;
        }
        
        int k = 0, lastNum = nums[0];

        for (int i = 0; i < nums.size(); i++) {
            if (nums[i] != val) {
                nums[k++] = nums[i];
            }
        }

        return k;
    }
};