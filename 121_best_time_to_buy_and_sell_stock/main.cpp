#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    int maxProfit(vector<int>& prices) {
        int result = 0;
        int minBuyPrice = prices[0];

        for (int i = 1; i < prices.size(); i++) {
            int sellPrice = prices[i];
            int profit = sellPrice - minBuyPrice;
            minBuyPrice = min(minBuyPrice, sellPrice);
            result = max(result, profit);
        }

        return result;
    }
};
