class Solution {
public:
    long long maxAlternatingSum(vector<int>& nums) {
        int n = nums.size();
        vector<long long> a(n);
        for (int i = 0; i < n; i++)
            a[i] = 1LL * nums[i] * nums[i];
        sort(a.begin(), a.end());
        long long ans = 0;
        int k = n / 2;
        for (int i = 0; i < k; i++)
            ans -= a[i];
        for (int i = k; i < n; i++)
            ans += a[i];
        return ans;
    }
};
