#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    int majorityElement(vector<int>& nums) {
        // result can be any number
        int count = 0, result = 0;

        for (auto num : nums) {
            if (count == 0) {
                result = num;
                count++;
            } else {
                if (result == num)
                    count++;
                else
                    count--;
            }
        }

        return result;
    }
};
