#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    vector<int> twoSum(vector<int>& nums, int target) {
        unordered_multimap<int, int> mp;
        for (int i = 0; i < nums.size(); i++) {
            mp.insert({nums[i], i});
        }

        sort(nums.begin(), nums.end());

        int l = 0, r = nums.size() - 1;
        while (l < r) {
            if (nums[l] + nums[r] < target) {
                l++;
            } else if (nums[l] + nums[r] > target) {
                r--;
            } else {
                if (nums[l] == nums[r]) {
                    return {mp.find(nums[l])->second, (++mp.find(nums[r]))->second};
                }
                return {mp.find(nums[l])->second, mp.find(nums[r])->second};
            }
        }

        return {};
    }
};
