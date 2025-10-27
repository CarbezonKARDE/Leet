class Solution {
    public int numberOfBeams(String[] bank) {
        long ans = 0;
        long prev = 0;
        for (String row : bank) {
            long cnt = 0;
            for (int i = 0; i < row.length(); ++i) {
                if (row.charAt(i) == '1') cnt++;
            }
            if (cnt > 0) {
                ans += prev * cnt;
                prev = cnt;
            }
        }
        return (int) ans;
    }
}
