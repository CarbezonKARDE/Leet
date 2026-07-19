class Solution {
    public String smallestSubsequence(String s) {
        int[] freq = new int[26];
        boolean[] seen = new boolean[26];
        for (char ch : s.toCharArray()) {
            freq[ch - 'a']++;
        }
        StringBuilder res = new StringBuilder();
        for (char ch : s.toCharArray()) {
            freq[ch - 'a']--;
            if (seen[ch - 'a']) continue;
            while (res.length() > 0 &&
                   res.charAt(res.length() - 1) > ch &&
                   freq[res.charAt(res.length() - 1) - 'a'] > 0) {
                seen[res.charAt(res.length() - 1) - 'a'] = false;
                res.deleteCharAt(res.length() - 1);
            }
            res.append(ch);
            seen[ch - 'a'] = true;
        }
        return res.toString();
    }
}
