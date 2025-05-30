class Solution {
    private static final int MOD = 1_000_000_007;
    public int numWays(String[] words, String target) {
        int n = target.length();
        int m = words[0].length();
        var freq = new int[m][26];
        for(var word : words) {
            for(int i=0; i<m; ++i){
                ++freq[i][word.charAt(i)-'a'];
            }            
        }
        var dp = new long[m];
        int c = target.charAt(0) - 'a';
        for(int j=0; j<=m-n; ++j) {
            dp[j] = freq[j][c];
            if(j>0) dp[j] += dp[j-1];
        }    
        for(int i=1; i<n; ++i) {
            c = target.charAt(i) - 'a';
            int max = m - n + i;
            for(int j=max; j>=i; --j) {
                dp[j] = dp[j-1] * freq[j][c];
                dp[j] %= MOD;                
            }
            for(int j=i+1; j<=max; ++j) {
                dp[j] += dp[j-1];
                dp[j] %= MOD;
            }
        }
        return (int)dp[m-1];
    }
}
