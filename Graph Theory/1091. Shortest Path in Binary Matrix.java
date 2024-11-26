class Solution {
    public int shortestPathBinaryMatrix(int[][] grid) {
        int n = grid.length;
        if(grid[0][0] == 1 || grid[n - 1][n - 1] == 1) return -1;
        if(n == 1 && grid[0][0] == 0) return 1;
        Deque<int[]> queue = new ArrayDeque<>(); 
        queue.add(new int[] {0, 0, 1});
        while(!queue.isEmpty()) {
            int[] curr = queue.pollFirst();
            int r = curr[0];
            int c = curr[1];
            int dist = curr[2];
            for(int i = r - 1; i <= r + 1; i++) {
                for(int j = c - 1; j <= c + 1; j++) {
                    if(i == -1 || i == n || j == -1 || j == n || grid[i][j] == 1) continue;
                    if(n - 1 == i && n - 1 == j) return dist + 1;
                    else {
                        queue.add(new int[] {i, j, dist + 1});
                        grid[i][j] = 1; 
                    }
                }
            }
        }
        return -1;
    }
}
