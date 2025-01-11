class Solution {
    public boolean canConstruct(String s, int k) {
        if (s.length() < k) return false;
        if (s.length() == k) return true;
        int oddCount = 0;
        for (char chr : s.toCharArray()) {
            oddCount ^= 1 << (chr - 'a');
        }
        return Integer.bitCount(oddCount) <= k;
    }
}
