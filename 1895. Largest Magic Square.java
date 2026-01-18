class Solution {
    public int largestMagicSquare(int[][] grid) {
        int n = grid.length;
        int m = grid[0].length;
        if (n < 2 || m < 2) return 1;
        long[][] row = new long[n][m + 1];
        long[][] col = new long[n + 1][m];
        long[][] d1 = new long[n + 1][m + 2];
        long[][] d2 = new long[n + 1][m + 2];
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < m; j++) {
                row[i][j + 1] = row[i][j] + grid[i][j];
                col[i + 1][j] = col[i][j] + grid[i][j];
                d1[i + 1][j + 1] = d1[i][j] + grid[i][j];
                d2[i + 1][j] = d2[i][j + 1] + grid[i][j];
            }
        }
        for (int k = Math.min(n, m); k > 1; k--) {
            for (int r = 0; r <= n - k; r++) {
                for (int c = 0; c <= m - k; c++) {
                    long target = row[r][c + k] - row[r][c];
                    if (d1[r + k][c + k] - d1[r][c] != target) continue;
                    if (d2[r + k][c] - d2[r][c + k] != target) continue;
                    boolean match = true;
                    for (int i = 0; i < k; i++) {
                        if (row[r + i][c + k] - row[r + i][c] != target) {
                            match = false;
                            break;
                        }
                    }
                    if (!match) continue;
                    for (int j = 0; j < k; j++) {
                        if (col[r + k][c + j] - col[r][c + j] != target) {
                            match = false;
                            break;
                        }
                    }
                    if (match) return k;
                }
            }
        }
        return 1;
    }
}
