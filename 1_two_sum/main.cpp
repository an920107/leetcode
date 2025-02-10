#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        unordered_map<int, vector<int>> mp;
        for (int i = 0; i < nums.size(); i++) {
            if (mp.find(nums[i]) == mp.end()) {
                mp[nums[i]] = vector<int>();
            }
            mp[nums[i]].emplace_back(i);
        }

        for (auto &[x, y] : mp) {
        }

        sort(nums.begin(), nums.end());

        for (int i = nums.size() - 1; i >= 0; i--) {
            int x = nums[i];
            int need = target - x;

            if (binary_search(nums.begin(), nums.begin() + i, need)) {
                if (x == need) {
                    return {mp[x][0], mp[need][1]};
                }
                return {mp[x][0], mp[need][0]};
            }
        }

        return {};
    }
};
