class Solution {
    public boolean isTrionic(int[] nums) {
        int n = nums.length;
        int peak = -1, valley = n;
        for (int i = 0; i < n - 1; i++) {
            if (peak == -1 && nums[i] >= nums[i + 1])
                peak = i;
            if (valley == n && nums[n - 1 - i] <= nums[n - 2 - i])
                valley = n - 1 - i;
            if (peak > 0 && valley < n - 1 && peak < valley)
                return isDecreasing(nums, peak, valley);
        }
        return false;
    }
    boolean isDecreasing(int[] nums, int a, int b) {
        for (int i = a; i < b; i++)
            if (nums[i] <= nums[i + 1])
                return false;
        return true;
    }
}
