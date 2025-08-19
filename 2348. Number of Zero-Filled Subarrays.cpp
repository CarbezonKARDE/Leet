class Solution {
public:
    long long zeroFilledSubarray(vector<int>& nums) {
        long long ans = 0;
        int n = nums.size(), curr = 0;
        for (int i = 0; i < n; i++) {
            if (nums[i] == 0) {
                curr++;
            } else {
                ans += 1LL * curr * (curr + 1) / 2;
                curr = 0;
            }
        }
        ans += 1LL * curr * (curr + 1) / 2;
        return ans;
    }
};
