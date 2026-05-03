#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    vector<vector<int>> permute(vector<int>& nums) {
        vector<vector<int>> result;

        sort(nums.begin(), nums.end());
        result.emplace_back(nums);

        while (next_permutation(nums.begin(), nums.end())) {
            result.emplace_back(nums);
        }
        return result;
    }
};
