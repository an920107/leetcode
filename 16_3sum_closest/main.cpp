#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    int threeSumClosest(vector<int>& nums, int target) {
        sort(nums.begin(), nums.end());

        int minDistance = INT32_MAX;
        int result = target;

        for (int i = 0; i < nums.size() - 1; i++) {
            int l = i + 1, r = nums.size() - 1;

            while (l < r) {
                int sum = nums[l] + nums[r] + nums[i];
                int distance = abs(sum - target);
                if (sum < target) {
                    if (distance < minDistance) {
                        minDistance = distance;
                        result = sum;
                    }
                    l++;
                } else if (sum > target) {
                    if (distance < minDistance) {
                        minDistance = distance;
                        result = sum;
                    }
                    r--;
                } else {
                    return target;
                }
            }
        }

        return result;
    }
};
