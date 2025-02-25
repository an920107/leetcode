#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    vector<int> searchRange(vector<int>& nums, int target) {
        if (!binary_search(nums.begin(), nums.end(), target)) {
            return {-1, -1};
        }

        auto first = lower_bound(nums.begin(), nums.end(), target);
        auto second = upper_bound(nums.begin(), nums.end(), target);

        return {(int)(first - nums.begin()), (int)(second - nums.begin() - 1)};
    }
};
