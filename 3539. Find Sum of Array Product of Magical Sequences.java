class Solution {
    private static final int MOD = 1_000_000_007;
    int n, k;
    int[][] comb;
    int[] nums;
    Integer[][][][] memo;
    public int magicalSum(int m, int k, int[] nums) {
        this.k = k;
        this.nums = nums;
        n = nums.length;
        this.comb = new int[m + 1][m + 1];
        comb(m, m);
        memo = new Integer[m + 1][k + 1][n][m + 1];
        return (int) dp(m, 0, 0, 0);
    }
    private int dp(int m, int curK, int i, int carry) {
        if(m < 0 || curK > k) {
            return 0;
        }
        if(i == n) {
            if(m == 0) {
                if(curK + Integer.bitCount(carry) == k) {
                    return 1;
                }
                return 0;
            }
            return 0;
        }
        if(memo[m][curK][i][carry] != null) {
            return memo[m][curK][i][carry];
        }
        long res = 0;
        for(int count = 0; count <= m; count++) {
            int newM = m - count;
            int temp = count + carry;
            int bitAdd = temp % 2;
            int newCarry = temp / 2;
            int newK = curK + bitAdd;
            long sub = dp(newM, newK, i + 1, newCarry);
            if(sub == 0) continue;
            long contribution = comb[m][count] * modPow(nums[i], count) % MOD;
            res = (res + sub  * contribution) % MOD;
        }
        return memo[m][curK][i][carry] = (int) res;
    }
    private long modPow(int a, int b) {
        int res = 1;
        while(b > 0) {
            if((b & 1) == 1) {
                res = mult(res, a);
            }
            a = mult(a, a);
            b /= 2;
        }
        return res;
    }
    private int mult(int a, int b) {
        return (int) (((long) a * b) % MOD);
    }
    private void comb(int n, int x) {
        comb[0][0] = 1;
        for(int i = 1; i <= n; i++) {
            for(int j = 0; j <= x; j++) {
                if(j == 0) {
                    comb[i][0] = 1;
                    continue;
                }
                comb[i][j] = comb[i - 1][j - 1] + comb[i - 1][j];
            }
        }
    }
}
