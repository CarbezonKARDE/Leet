class Solution {
public:
    int numberOfSubmatrices(vector<vector<char>>& grid) {
        int m = grid.size();
        int n = grid[0].size();
        vector<int> ox(n, 0);
        vector<int> oy(n, 0);
        int res = 0;
        for (int i = 0; i < m; i++) {
            int rowX = 0, rowY = 0;
            for (int j = 0; j < n; j++) {
                if (grid[i][j] == 'X')
                    rowX++;
                else if (grid[i][j] == 'Y')
                    rowY++;
                ox[j] += rowX;
                oy[j] += rowY;
                if (ox[j] == oy[j] && ox[j] > 0)
                    res++;
            }
        }
        return res;
    }
};
