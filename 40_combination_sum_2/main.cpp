#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    vector<vector<int>> combinationSum2(vector<int>& candidates, int target) {
        vector<vector<int>> result;

        sort(candidates.begin(), candidates.end());

        function<void(int, vector<int>, int)> recursive = [&](int target, vector<int> chosen, int startIndex) {
            if (target == 0) {
                result.emplace_back(chosen);
                return;
            }

            for (int i = startIndex; i < candidates.size(); i++) {
                int x = candidates[i];

                if (i > startIndex && x == candidates[i - 1]) {
                    continue;
                }

                if (target < x) return;

                chosen.emplace_back(x);
                recursive(target - x, chosen, i + 1);
                chosen.pop_back();
            }
        };

        recursive(target, {}, 0);

        return result;
    }
};

int main() {
    vector<int> v{10, 1, 2, 7, 6, 1, 5};
    Solution().combinationSum2(v, 8);

    return 0;
}
