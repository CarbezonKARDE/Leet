class Solution {
    int dfs(List<List<Integer>> adj, boolean[] visited, int src) {
        int change = 0;
        visited[src] = true;
        for(int node : adj.get(src)) {
            if(!visited[Math.abs(node)]) {
                change += (node > 0 ? 1 : 0);
                change += dfs(adj, visited, Math.abs(node));
            }
        }
        return change;
    }
    public int minReorder(int n, int[][] connections) {
        List<List<Integer>> adj = new ArrayList<>();
        for(int i = 0; i < n; i++) adj.add(new ArrayList<>());
        for(int conn[] : connections) {
            int src = conn[0];
            int dest = conn[1];
            adj.get(src).add(dest);
            adj.get(dest).add(-src);
        }
        return dfs(adj, new boolean[n], 0);
    }
}
