class Solution {
    public int minExtraChar(String s, String[] dict) {
        int n = s.length();
        HashSet<String> set = new HashSet<>();
        for(int i = 0 ; i < dict.length; i++){
            set.add(dict[i]);
        }
        int[] dp = new int[n];
        Arrays.fill(dp, -1);
        return solve(0, set, s, dp);
    }
    int solve(int ind, HashSet<String> set, String s, int[] dp){
        if(ind >= s.length()) return 0;
        if(dp[ind] != -1) return dp[ind];
        int ans = Integer.MAX_VALUE;
        String sub = "";
        for(int i = 0; i + ind < s.length(); i++){
            sub = s.substring(ind, ind + i + 1);

            if(set.contains(sub)){
                ans = Math.min(ans, solve(ind+i+1, set, s, dp));
            }
            ans = Math.min(ans,i + 1 + solve(ind+i+1, set, s, dp));
        } 
        return dp[ind] = ans;
    }
}
