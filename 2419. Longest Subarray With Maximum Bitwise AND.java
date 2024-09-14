class Solution {
    public int longestSubarray(int[] nums) {
        int max = nums[0];
        for(int ele : nums) {
            if(ele > max) max = ele;
        }
        int result = 0;
        int seq = 0;
        for(int ele : nums) {
            if(ele == max) {
                seq++;
                if(seq > result) result = seq;
            }
            else seq = 0;
        }
        return result;
    }
}
