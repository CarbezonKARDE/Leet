class Solution {
    static final long INF = -1000000000000000L;
    int n, budget;
    int[] present, future, capability;
    List<Integer>[] tree;
    long[][] dp0, dp1;
    long cap(int u) {
        long sum = present[u];
        for (int v : tree[u]) {
            sum += cap(v);
        }
        capability[u] = (int) Math.min(budget, sum);
        return sum;
    }
    long[] merge(long[] a, long[] b) {
        int lenA = a.length - 1;
        int lenB = b.length - 1;
        int total = Math.min(budget, lenA + lenB);
        long[] c = new long[total + 1];
        Arrays.fill(c, INF);
        for (int i = 0; i <= Math.min(lenA, total); i++) {
            if (a[i] == INF) 
                continue;
            for (int j = 0; j <= Math.min(lenB, total - i); j++) {
                if (b[j] == INF) 
                    continue;
                c[i + j] = Math.max(c[i + j], a[i] + b[j]);
            }
        }
        return c;
    }
    void dfs(int u) {
        for (int v : tree[u]) dfs(v);
        int capU = capability[u];
        long[] skip = new long[capU + 1];
        long[] base = new long[capU + 1];
        Arrays.fill(skip, INF);
        Arrays.fill(base, INF);
        skip[0] = base[0] = 0;
        for (int v : tree[u]) {
            skip = merge(skip, dp0[v]);
            base = merge(base, dp1[v]);
        }
        dp0[u] = compute(u, 0, skip, base);
        dp1[u] = compute(u, 1, skip, base);
    }
    long[] compute(int u, int parentBought, long[] skip, long[] base) {
        int price = parentBought == 1 ? present[u] / 2 : present[u];
        long profit = future[u] - price;
        long[] res = skip.clone();
        for (int b = price; b < res.length; b++) {
            if (base[b - price] != INF) {
                res[b] = Math.max(res[b], base[b - price] + profit);
            }
        }
        return res;
    }
    public int maxProfit(int n, int[] present, int[] future, int[][] hierarchy, int budget) {
        this.n = n;
        this.budget = budget;
        this.present = present;
        this.future = future;
        tree = new ArrayList[n];
        for (int i = 0; i < n; i++) tree[i] = new ArrayList<>();
        int[] indeg = new int[n];
        for (int[] e : hierarchy) {
            int u = e[0] - 1, v = e[1] - 1;
            tree[u].add(v);
            indeg[v]++;
        }
        int root = 0;
        for (int i = 0; i < n; i++)
            if (indeg[i] == 0) root = i;
        capability = new int[n];
        cap(root);
        dp0 = new long[n][];
        dp1 = new long[n][];
        dfs(root);
        long ans = 0;
        for (long x : dp0[root]) ans = Math.max(ans, x);
        return (int) ans;
    }
}
