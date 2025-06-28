
class Solution {
    public int[] maxSubsequence(int[] nums, int k) {
        int[][] valueWithIndex = new int[nums.length][2];
        for (int i = 0; i < nums.length; i++) {
            valueWithIndex[i] = new int[]{nums[i], i};
        }
        Arrays.sort(valueWithIndex, (a, b) -> b[0] - a[0]);
        Arrays.sort(valueWithIndex, 0, k, (a, b) -> a[1] - b[1]);
        int[] result = new int[k];
        for (int i = 0; i < k; i++) {
            result[i] = valueWithIndex[i][0];
        }
        return result;
    }
}
