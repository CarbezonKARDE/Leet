[⚠️ Suspicious Content] class Solution {
public:
    int maxAdjacentDistance(vector<int>& nums) {
        int n = nums.size();
        int maxa = abs(nums[0] - nums[n - 1]);
        for (int i = 0; i < n - 1; i++) {
            maxa = max(maxa, abs(nums[i] - nums[i + 1]));
        }
        return maxa;
    }
};
