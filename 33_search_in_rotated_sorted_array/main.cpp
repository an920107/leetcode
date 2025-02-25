#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    int recursive(vector<int>& nums, int target, int left, int right) {
        int mid = (right + left) / 2;

        if (target == nums[mid]) {
            return mid;
        }

        if (left >= right) {
            return -1;
        }

        if (left + 1 == right) {
            if (target == nums[left]) return left;
            if (target == nums[right]) return right;
        }

        // the left part is sorted && target is in the left part
        if (nums[left] <= target && target <= nums[mid]) {
            if (binary_search(nums.begin() + left, nums.begin() + mid + 1, target)) {
                return lower_bound(nums.begin() + left, nums.begin() + mid + 1, target) - nums.begin();
            }
            return -1;
        }

        // the right part is sorted && target is in the right part
        if (nums[mid] <= target && target <= nums[right]) {
            if (binary_search(nums.begin() + mid, nums.begin() + right + 1, target)) {
                return lower_bound(nums.begin() + mid, nums.begin() + right + 1, target) - nums.begin();
            }
            return -1;
        }

        // target is not in a sorted part
        if (nums[left] < nums[mid]) {  // the right part is not sorted
            return recursive(nums, target, mid, right);
        } else {  // the left part is not sorted
            return recursive(nums, target, left, mid);
        }
    }

    int search(vector<int>& nums, int target) {
        return recursive(nums, target, 0, nums.size() - 1);
    }
};

int main() {
    vector<int> vec{3, 1};
    Solution().search(vec, 1);
    return 0;
}
