class Solution {
    public int longestSubarray(int[] nums) {
        int l = 0, r = 0, d = -1, max = 0, len = nums.length;
        while (r < len) {
            if (nums[r] == 0) {
                if (d == -1) {
                    d = r;
                    r++;
                } else {
                    max = Math.max(max, r - l - 1);
                    l = d + 1;
                    d = -1;
                }
            } else {
                r++;
            }
        }
        return Math.max(max, r - l - 1);
    }
}
