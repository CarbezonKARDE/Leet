class Solution {
    public List<String> maxNumOfSubstrings(String s) {
        int n = s.length();
        int[] first = new int[26];
        int[] last = new int[26];
        Arrays.fill(first, n);
        Arrays.fill(last, -1);
        for (int i = 0; i < n; i++) {
            int c = s.charAt(i) - 'a';
            first[c] = Math.min(first[c], i);
            last[c] = i;
        }
        List<int[]> intervals = new ArrayList<>();
        for (int c = 0; c < 26; c++) {
            if (last[c] == -1) continue;
            int l = first[c];
            int r = last[c];
            boolean valid = true;
            for (int i = l; i <= r; i++) {
                int x = s.charAt(i) - 'a';
                if (first[x] < l) {
                    valid = false;
                    break;
                }
                r = Math.max(r, last[x]);
            }
            if (valid)
                intervals.add(new int[]{r, l});
        }
        intervals.sort((a, b) -> {
            if (a[0] != b[0]) return Integer.compare(a[0], b[0]);
            return Integer.compare(a[1], b[1]);
        });
        List<String> ans = new ArrayList<>();
        int prevEnd = -1;
        for (int[] iv : intervals) {
            int r = iv[0], l = iv[1];
            if (l > prevEnd) {
                ans.add(s.substring(l, r + 1));
                prevEnd = r;
            }
        }
        return ans;
    }
}
