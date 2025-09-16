class Solution {
    public List<Integer> replaceNonCoprimes(int[] nums) {
        int n = nums.length;
        int[] stack = new int[n + 1];
        stack[0] = 1;
        int j = 0;
        for(int num : nums) {
            int factor = gcd(stack[j], num);
            while(factor > 1) {
                num = num / factor * stack[j--];
                factor = gcd(stack[j], num);
            }
            stack[++j] = num;
        }
        List<Integer> list = new ArrayList<>();
        for(int i = 1; i <= j; i++) list.add(stack[i]);
        return list;
    }
    private int gcd(int x, int y) {
        return x == 0 ? y : gcd(y % x, x);
    }

}
