#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    int maxSubArray(vector<int>& nums) {
        return maxSubArray(nums, 0, nums.size() - 1);
    }

    int maxSubArray(vector<int>& nums, int leftIndex, int rightIndex) {
        if (leftIndex > rightIndex) return INT32_MIN;

        int mid = (leftIndex + rightIndex) / 2;
        int leftMaxSum = 0, rightMaxSum = 0;

        // left half side sum
        for (int i = mid - 1, currentSum = 0; i >= leftIndex; i--) {
            currentSum += nums[i];
            leftMaxSum = max(leftMaxSum, currentSum);
        }

        // right half side sum
        for (int i = mid + 1, currentSum = 0; i <= rightIndex; i++) {
            currentSum += nums[i];
            rightMaxSum = max(rightMaxSum, currentSum);
        }

        return max(max(maxSubArray(nums, leftIndex, mid - 1),
                       maxSubArray(nums, mid + 1, rightIndex)),
                   leftMaxSum + nums[mid] + rightMaxSum);
    }
};
