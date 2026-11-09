class Solution {
    private record State(int time, int id) {}; 
    public List<Integer> busiestServers(int k, int[] arrival, int[] load) {
        TreeSet<Integer> available = new TreeSet<>(); 
        for(int i = 0; i < k; i++) available.add(i); 
        PriorityQueue<State> busy = new PriorityQueue<>((a, b) -> Integer.compare(a.time, b.time)); 
        int cnt[] = new int[k];
        int mx = 0; 
        for(int i = 0; i < arrival.length; i++) {
            while(!busy.isEmpty() && busy.peek().time <= arrival[i]) {
                int id = busy.poll().id;
                available.add(id);
            }
            if(available.isEmpty()) continue;
            Integer serverId = available.ceiling(i % k); 
            if(serverId == null) serverId = available.first(); 
            cnt[serverId]++; 
            mx = Math.max(mx, cnt[serverId]); 
            busy.add(new State(arrival[i] + load[i], serverId)); 
            available.remove(serverId);
        }
        List<Integer> ans = new ArrayList<>(); 
        for(int i = 0; i < k; i++) if(cnt[i] == mx) ans.add(i); 
        return ans; 
    }
}
