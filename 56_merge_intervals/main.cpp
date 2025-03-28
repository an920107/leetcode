#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    vector<vector<int>> merge(vector<vector<int>>& intervals) {
        sort(intervals.begin(), intervals.end(), [](vector<int> a, vector<int> b) {
            if (b[0] > a[0]) return true;
            if (b[0] < a[0]) return false;
            return b[1] > a[1];
        });

        vector<vector<int>> result;

        for (int i = 0; i < intervals.size(); i++) {
            int currentStart = intervals[i][0], currentEnd = intervals[i][1];

            int j = i + 1;
            while (j < intervals.size() &&
                   currentStart <= intervals[j][0] &&
                   intervals[j][0] <= currentEnd) {
                currentEnd = max(currentEnd, intervals[j][1]);
                j++;
            }
            i = j - 1;

            vector<int> toPush{currentStart, currentEnd};
            result.emplace_back(toPush);
        }

        return result;
    }
};
