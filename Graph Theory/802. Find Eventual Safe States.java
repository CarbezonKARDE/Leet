class Solution {
    public List<Integer> eventualSafeNodes(int[][] graph) {
        int n = graph.length;
        List<Integer> arr[] = new ArrayList[n];
        for(int i=0; i<n; i++) arr[i] = new ArrayList<>();
        int indegree[] = new int[n];
        for(int i=0; i<n; i++){
            for(int j=0; j<graph[i].length; j++){
                arr[graph[i][j]].add(i);
                indegree[i]++;
            }
        }
        int safe[] = new int[n];
        Queue<Integer> q = new LinkedList<>();
        for(int i=0; i<n; i++) if(indegree[i]==0) q.add(i);
        while(q.size()!=0){
            int a = q.remove();
            safe[a] = 1;
            for(var x : arr[a]){
                indegree[x]--;
                if(indegree[x]==0) q.add(x);
            }
        }
        List<Integer> ans = new ArrayList<>();
        for(int i=0; i<n; i++){
            if(safe[i]==1) ans.add(i);
        }
        return ans;
    }
}
