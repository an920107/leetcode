#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    bool isValid(string s) {
        stack<char> stk;

        for (auto c : s) {
            switch (c) {
                case '(':
                case '[':
                case '{':
                    stk.emplace(c);
                    break;
                case ')':
                    if (stk.empty() || '(' != stk.top()) {
                        return false;
                    }
                    stk.pop();
                    break;
                case ']':
                    if (stk.empty() || '[' != stk.top()) {
                        return false;
                    }
                    stk.pop();
                    break;
                case '}':
                    if (stk.empty() || '{' != stk.top()) {
                        return false;
                    }
                    stk.pop();
                    break;
            }
        }

        if (!stk.empty()) {
            return false;
        }

        return true;
    }
};
