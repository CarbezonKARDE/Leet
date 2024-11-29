class Solution {
    public List<Integer> findSmallestSetOfVertices(int n, List<List<Integer>> edges) {
        int[] visit=new int[n]; 
        for(int i=0;i<edges.size();i++){
            visit[edges.get(i).get(1)]++;
        } 
        List<Integer> ans=new ArrayList<>();
        for(int i=0;i<n;i++){
            if(visit[i]==0)ans.add(i);
        }
        return ans;
    }
}
