#include <bits/stdc++.h>
using namespace std;

class Solution {
   public:
    bool checkRepeat(bitset<9>& bs, char c) {
        if (c != '.') {
            int d = c - '1';
            if (bs[d]) return true;
            bs.flip(d);
        }
        return false;
    }

    bool isRowValid(vector<vector<char>>& board, int row) {
        bitset<9> bs(0);
        for (auto c : board[row]) {
            if (checkRepeat(bs, c)) return false;
        }
        return true;
    }

    bool isColValid(vector<vector<char>>& board, int col) {
        bitset<9> bs(0);
        for (int i = 0; i < 9; i++) {
            char c = board[i][col];
            if (checkRepeat(bs, c)) return false;
        }
        return true;
    }

    bool isSecValid(vector<vector<char>>& board, int row, int col) {
        bitset<9> bs(0);
        for (int i = row; i < row + 3; i++) {
            for (int j = col; j < col + 3; j++) {
                char c = board[i][j];
                if (checkRepeat(bs, c)) return false;
            }
        }
        return true;
    }

    bool isValidSudoku(vector<vector<char>>& board) {
        for (int i = 0; i < 9; i++) {
            if (!isRowValid(board, i)) return false;
        }

        for (int i = 0; i < 9; i++) {
            if (!isColValid(board, i)) return false;
        }

        for (int i = 0; i < 9; i += 3) {
            for (int j = 0; j < 9; j += 3) {
                if (!isSecValid(board, i, j)) return false;
            }
        }

        return true;
    }
};
