class Solution {
    public String robotWithString(String s) {
        int[] freq = new int[26];
        int j = 0, i = 0;
        StringBuilder sb = new StringBuilder();
        StringBuilder sb2 = new StringBuilder();
        for (char c : s.toCharArray()) {
            freq[c - 'a']++;
        }
        while (i < 26 && freq[i] == 0) {
            i++;
        }
        for (char c : s.toCharArray()) {
            if (c - 'a' == i) {
                sb.append(c);
                freq[c - 'a']--;
                if (freq[c - 'a'] == 0) {
                    while (i < 26 && freq[i] == 0) {
                        i++;
                    }
                    while (i < 26 && sb2.length() > 0 && sb2.charAt(sb2.length() - 1) <= i + 'a') 
                        sb.append(sb2.charAt(sb2.length() - 1));
                        sb2.deleteCharAt(sb2.length() - 1);
                    }
                }
            } else {
                sb2.append(c);
                freq[c - 'a']--;
            }
        }
        sb2.reverse();
        sb.append(sb2);
        return sb.toString();
    }
}
