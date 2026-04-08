class Solution {
public:
    int xorAfterQueries(vector<int>& nums, vector<vector<int>>& queries) {
        const int MOD = 1e9 + 7;
        for (const auto& q : queries) {
            int l = q[0];
            int r = q[1];
            int k = q[2];
            int v = q[3];
            for (int idx = l; idx <= r; idx += k) {
                nums[idx] = (1LL * nums[idx] * v) % MOD;
            }
        }
        int result = 0;
        for (int val : nums) {
            result ^= val;
        }
        return result;
    }
};
