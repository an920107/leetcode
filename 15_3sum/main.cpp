#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    vector<vector<int>> threeSum(vector<int> &nums) {
        set<vector<int>> st;

        sort(nums.begin(), nums.end());

        for (int i = 0; i < nums.size(); i++) {
            int target = -nums[i];
            int l = i + 1, r = nums.size() - 1;

            while (l < r) {
                if (nums[l] + nums[r] < target) {
                    l++;
                } else if (nums[l] + nums[r] > target) {
                    r--;
                } else {
                    st.insert({nums[i], nums[l], nums[r]});
                    l++;
                }
            }
        }

        vector<vector<int>> result;
        for (auto &v : st) {
            result.push_back(v);
        }
        return result;
    }
};
