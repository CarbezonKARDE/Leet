class Solution {
public:
    struct room {
        int row;
        int col;
        int distance;
    };
    int nearestExit(vector<vector<char>>& maze, vector<int>& entrance) {
        int m = maze.size();
        int n = maze[0].size();
        vector<vector<bool>> visited(m,vector<bool>(n,false));
        queue<room> q;
        q.push({entrance[0], entrance[1], 0});
        visited[entrance[0]][entrance[1]] = true;
        while (!q.empty()) {
            auto e = q.front();
            int row = e.row;
            int col = e.col;
            int dis = e.distance;
            q.pop();
            if ((row == 0 || col == 0 || row == m-1 || col == n-1) && !(row == entrance[0] && col == entrance[1])) {
                return dis;
            }
            if ((row - 1 >= 0) && (maze[row - 1][col] == '.') && (!visited[row - 1][col])) {
                visited[row - 1][col] = true;
                q.push({row - 1, col, dis + 1});
            }
            if ((col - 1 >= 0) && (maze[row][col - 1] == '.') && (!visited[row][col - 1])) {
                visited[row][col - 1] = true;
                q.push({row, col - 1, dis + 1});
            }
            if ((row + 1 < m) && (maze[row + 1][col] == '.') && (!visited[row + 1][col])) {
                visited[row + 1][col] = true;
                q.push({row + 1, col, dis + 1});
            }
            if ((col + 1 < n) && (maze[row][col + 1] == '.') && (!visited[row][col + 1])) {
                visited[row][col + 1] = true;
                q.push({row, col + 1, dis + 1});
            }
        }
        return -1;
    }
};
auto init = []()
{ 
    ios::sync_with_stdio(0);
    cin.tie(0);
    cout.tie(0);
    return 'c';
}();
