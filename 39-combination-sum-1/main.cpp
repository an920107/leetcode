#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
        sort(candidates.begin(), candidates.end());

        vector<vector<int>> result;

        function<void(int, vector<int>)> recursive = [&](int target, vector<int> chosen) {
            if (target < 0) return;
            
            if (target == 0) {
                result.emplace_back(chosen);
                return;
            }

            for (auto x : candidates) {
                if (chosen.size() > 0 && x > chosen.back()) {
                    return;
                }

                chosen.emplace_back(x);
                recursive(target - x, chosen);
                chosen.pop_back();
            }
        };

        recursive(target, {});

        return result;
    }
};
