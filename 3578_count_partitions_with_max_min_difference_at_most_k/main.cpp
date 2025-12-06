#include <bits/stdc++.h>
using namespace std;

const int MOD = 1000000007;

class Solution {
   public:
    int countPartitions(vector<int>& nums, int k) {
        vector<int> dp(nums.size() + 1);
        dp[0] = 1;
        long long acc = 1;
        size_t left = 0, right = 0;
        multiset<int> s;

        while (right < nums.size()) {
            s.insert(nums[right]);

            while (*(--s.end()) - *s.begin() > k) {
                acc -= dp[left];
                if (acc < 0) {
                    acc += MOD;
                }
                auto iter = s.find(nums[left]);
                s.erase(iter);
                left++;
            }

            dp[right + 1] = acc;
            acc *= 2;
            acc %= MOD;

            right++;
        }

        return dp.back();
    }
};

int main(int argc, char const* argv[]) {
    vector<int> nums{9, 4, 1, 3, 7};
    Solution().countPartitions(nums, 4);
    return 0;
}
