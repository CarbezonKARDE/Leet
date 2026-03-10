class Solution {
    private static final int MOD = (int) Math.pow(10, 9) + 7;
    public int numberOfStableArrays(int zero, int one, int limit) {
        if(zero > one) {
            return numberOfStableArrays(one, zero, limit);
        }
        int[] zeroPartitions = getNumPartitions(zero, limit, zero);
        int[] onePartitions = getNumPartitions(one, limit, zero+1);
        long res = 0;
        for(int i = 1; i <= zero; i++) {
            long oneChoices = (i > 1 ? onePartitions[i-1] : 0) + (long) 2*onePartitions[i] + (i+1 <= one ? onePartitions[i+1] : 0);
            oneChoices %= MOD;
            res += zeroPartitions[i] * oneChoices;
            res %= MOD;
        }
        return (int) res;
    }
    private int[] getNumPartitions(int n, int limit, int maxGroups) {
        int[] res = new int[maxGroups+1];
        int[] prev = new int[n+1];
        int[] cur = new int[n+1];
        prev[0] = 1;
        for(int numGroups = 1; numGroups <= Math.min(n, maxGroups); numGroups++) {
            cur[0] = 0;
            for(int sum = 1; sum <= n; sum++) {
                cur[sum] = cur[sum-1] + prev[sum-1];
                if(sum > limit) {
                    cur[sum] -= prev[sum-limit-1];
                }
                if(cur[sum] < 0) {
                    cur[sum] += MOD;
                }
                else if(cur[sum] >= MOD) {
                    cur[sum] -= MOD;
                }
            }
            res[numGroups] = cur[n];
            int[] temp = prev;
            prev = cur;
            cur = temp;
        }
        return res;
    }
}
