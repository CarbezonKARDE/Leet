class Solution {
    public boolean findSafeWalk(List<List<Integer>> grid, int health) {
        int m = grid.size();
        int n = grid.get(0).size();

        int[][] dist = new int[m][n];
        for (int i = 0; i < m; i++) {
            java.util.Arrays.fill(dist[i], Integer.MAX_VALUE);
        }
        java.util.ArrayDeque<int[]> dq = new java.util.ArrayDeque<>();
        dist[0][0] = grid.get(0).get(0);
        dq.offerFirst(new int[]{0, 0});
        int[] dir = {-1, 0, 1, 0, -1};
        while (!dq.isEmpty()) {
            int[] cur = dq.pollFirst();
            int x = cur[0];
            int y = cur[1];
            for (int k = 0; k < 4; k++) {
                int nx = x + dir[k];
                int ny = y + dir[k + 1];
                if (nx < 0 || ny < 0 || nx >= m || ny >= n)
                    continue;
                int newCost = dist[x][y] + grid.get(nx).get(ny);
                if (newCost < dist[nx][ny]) {
                    dist[nx][ny] = newCost;
                    if (grid.get(nx).get(ny) == 0)
                        dq.offerFirst(new int[]{nx, ny});
                    else
                        dq.offerLast(new int[]{nx, ny});
                }
            }
        }
        return dist[m - 1][n - 1] < health;
    }
}
