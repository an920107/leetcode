#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    vector<vector<int>> fourSum(vector<int>& nums, int target) {
        set<vector<int>> resultSet;

        sort(nums.begin(), nums.end());

        for (int i = 0; i < nums.size(); i++) {
            for (int j = i + 1; j < nums.size(); j++) {
                int l = j + 1, r = nums.size() - 1;

                while (l < r) {
                    long long sum = (long long)nums[i] + nums[j] + nums[l] + nums[r];
                    if (sum < target) {
                        l++;
                    } else if (sum > target) {
                        r--;
                    } else {
                        resultSet.insert({nums[i], nums[j], nums[l], nums[r]});
                        l++;
                    }
                }
            }
        }

        vector<vector<int>> result;
        for (auto& e : resultSet) {
            result.emplace_back(e);
        }

        return result;
    }
};
