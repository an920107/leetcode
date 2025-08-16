#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    vector<vector<int>> insert(vector<vector<int>>& intervals, vector<int>& newInterval) {
        if (intervals.empty()) return {newInterval};

        vector<vector<int>> result;

        if (newInterval[1] < intervals.front()[0]) {
            result.emplace_back(newInterval);
        }

        for (int i = 0; i < intervals.size(); i++) {
            // new interval 與 intervals[i] 有交集
            if ((intervals[i][0] <= newInterval[0] && newInterval[0] <= intervals[i][1]) ||
                (newInterval[0] <= intervals[i][0] && intervals[i][0] <= newInterval[1])) {
                vector<int> intervalToPush(intervals[i]);
                intervalToPush[0] = min(newInterval[0], intervalToPush[0]);
                for (; i < intervals.size(); i++) {
                    if (newInterval[1] < intervals[i][0]) break;
                    intervalToPush[1] = max(newInterval[1], intervals[i][1]);
                }

                result.emplace_back(intervalToPush);
                i--;
            } else {
                result.emplace_back(intervals[i]);
                // new interval 跟前後兩個 intervals[i], intervals[i + 1] 沒有交集
                if (intervals[i][1] < newInterval[0] &&
                    (i == intervals.size() - 1 || newInterval[1] < intervals[i + 1][0])) {
                    result.emplace_back(newInterval);
                }
            }
        }

        return result;
    }
};
