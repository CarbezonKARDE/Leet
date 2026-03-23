class Solution {
    int n, m;
    int MOD = 1000000007;
    boolean[][] vis;
    long[][][] dp;
    public long[] solve(int i, int j, int[][] grid) {
        if (i == n - 1 && j == m - 1) {
            return new long[]{grid[i][j], grid[i][j]};
        }
        if (vis[i][j]) {
            return new long[]{dp[i][j][0], dp[i][j][1]};
        }
        vis[i][j] = true;
        long mini = Long.MAX_VALUE;
        long maxi = Long.MIN_VALUE;
        if (i + 1 < n) {
            long[] down = solve(i + 1, j, grid);
            long a = down[0] * grid[i][j];
            long b = down[1] * grid[i][j];
            maxi = Math.max(maxi, Math.max(a, b));
            mini = Math.min(mini, Math.min(a, b));
        }
        if (j + 1 < m) {
            long[] right = solve(i, j + 1, grid);
            long a = right[0] * grid[i][j];
            long b = right[1] * grid[i][j];
            maxi = Math.max(maxi, Math.max(a, b));
            mini = Math.min(mini, Math.min(a, b));
        }
        dp[i][j][0] = maxi;
        dp[i][j][1] = mini;
        return new long[]{maxi, mini};
    }
    public int maxProductPath(int[][] grid) {
        n = grid.length;
        m = grid[0].length;
        vis = new boolean[n][m];
        dp = new long[n][m][2];
        long[] ans = solve(0, 0, grid);
        long res = ans[0];
        if (res < 0) return -1;
        return (int)(res % MOD);
    }
}
