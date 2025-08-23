class Solution {
public:
    int horizon(vector<vector<int>>& grid, int l, int r, int u, int d){
        int left = -1, right = -1, up = -1, down = -2, res = INT_MAX;
        vector<int> prefix(r + 2);
        for(int j=l; j<=r; j++){
            for(int i=u; i<=d; i++){
                if(grid[i][j] == 0) continue;
                if(left == -1) left = j;
                if(up == -1) up = i;
                left = min(left, j);
                right = max(right, j);
                up = min(up, i);
                down = max(down, i);
            }
            prefix[j+1] = (down - up + 1) * (right - left + 1);
        }
        left = -1, right = -1, up = -1, down = -2, res = prefix[r+1];
        for(int j=r; j>=l; j--){
            for(int i=u; i<=d; i++){
                if(grid[i][j] == 0) continue;
                if(left == -1) left = j;
                if(up == -1) up = i;
                left = min(left, j);
                right = max(right, j);
                up = min(up, i);
                down = max(down, i);
            }
            res = min(res, prefix[j] + (down - up + 1) * (right - left + 1));
        }
        return res;
    }
    int vertical(vector<vector<int>>& grid, int l, int r, int u, int d){
        int left = -1, right = -1, up = -1, down = -2, res = INT_MAX;
        vector<int> prefix(d + 2);
        for(int i=u; i<=d; i++){
            for(int j=l; j<=r; j++){
                if(grid[i][j] == 0) continue;
                if(left == -1) left = j;
                if(up == -1) up = i;
                left = min(left, j);
                right = max(right, j);
                up = min(up, i);
                down = max(down, i);
            }
            prefix[i+1] = (down - up + 1) * (right - left + 1);
        }
        left = -1, right = -1, up = -1, down = -2, res = prefix[d+1];
        for(int i=d; i>=u; i--){
            for(int j=l; j<=r; j++){
                if(grid[i][j] == 0) continue;
                if(left == -1) left = j;
                if(up == -1) up = i;
                left = min(left, j);
                right = max(right, j);
                up = min(up, i);
                down = max(down, i);
            }
            res = min(res, prefix[i] + (down - up + 1) * (right - left + 1));
        }
        return res;
    }
    int minimumSum(vector<vector<int>>& grid) {
        int n = grid.size(), m = grid[0].size(), res = min(horizon(grid, 0, m-1, 0, n-1), vertical(grid, 0, m-1, 0, n-1));
        int left = -1, right = -1, up = -1, down = -2;
        for(int i=0; i<n; i++){
            for(int j=0; j<m; j++){
                if(grid[i][j] == 0) continue;
                if(left == -1) left = j;
                if(up == -1) up = i;
                left = min(left, j);
                right = max(right, j);
                up = min(up, i);
                down = max(down, i);
            }
            auto area = (down - up + 1) * (right - left + 1);
            res = min(res, min(horizon(grid, 0, m-1, i+1, n-1), vertical(grid, 0, m-1, i+1, n-1)) + area);
        }
        left = -1, right = -1, up = -1, down = -2;
        for(int i=n-1; i>=0; i--){
            for(int j=0; j<m; j++){
                if(grid[i][j] == 0) continue;
                if(left == -1) left = j;
                if(up == -1) up = i;
                left = min(left, j);
                right = max(right, j);
                up = min(up, i);
                down = max(down, i);
            }
            auto area = (down - up + 1) * (right - left + 1);
            res = min(res, min(horizon(grid, 0, m-1, 0, i-1), vertical(grid, 0, m-1, 0, i-1)) + area);
        }
        left = -1, right = -1, up = -1, down = -2;
        for(int j=0; j<m; j++){
            for(int i=0; i<n; i++){
                if(grid[i][j] == 0) continue;
                if(left == -1) left = j;
                if(up == -1) up = i;
                left = min(left, j);
                right = max(right, j);
                up = min(up, i);
                down = max(down, i);
            }
            auto area = (down - up + 1) * (right - left + 1);
            res = min(res, min(horizon(grid, j+1, m-1, 0, n-1), vertical(grid, j+1, m-1, 0, n-1)) + area);
        }
        left = -1, right = -1, up = -1, down = -2;
        for(int j=m-1; j>=0; j--){
            for(int i=0; i<n; i++){
                if(grid[i][j] == 0) continue;
                if(left == -1) left = j;
                if(up == -1) up = i;
                left = min(left, j);
                right = max(right, j);
                up = min(up, i);
                down = max(down, i);
            }
            auto area = (down - up + 1) * (right - left + 1);
            res = min(res, min(horizon(grid, 0, j-1, 0, n-1), vertical(grid, 0, j-1, 0, n-1)) + area);
        }
        return res;
    }
};
