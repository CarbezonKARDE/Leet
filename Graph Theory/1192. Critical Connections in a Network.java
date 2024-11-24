class Solution {
    int time=0;
    List<List<Integer>> bridges = new ArrayList<>();
    List<Integer>[] graph;
    public List<List<Integer>> criticalConnections(int n, List<List<Integer>> connections) {
        int[] visitTime = new int[n];
        int[] lowestTime = new int[n];
        graph = new ArrayList[n];
        Arrays.fill(visitTime, -1);
        for (int i=0; i<n; i++) {
            graph[i] = new ArrayList<>();
        }
        for (List<Integer> connection : connections) {
            int a = connection.get(0);
            int b = connection.get(1);
            graph[a].add(b);
            graph[b].add(a);
        }
        dfs(0, 0, visitTime, lowestTime);
        return bridges;
    }
    void dfs(int v, int parent, int[] visitTime, int[] lowestTime) {
        time++;
        visitTime[v] = time;
        lowestTime[v] = time;
        for (Integer u : graph[v]) {
            if (u == parent) continue;
            if (visitTime[u] == -1) {
                dfs(u, v, visitTime, lowestTime);
            }
            lowestTime[v] = Math.min(lowestTime[v], lowestTime[u]);
            if (lowestTime[u] > visitTime[v]) {
                bridges.add(Arrays.asList(u, v));
            }
        }
    }
}
