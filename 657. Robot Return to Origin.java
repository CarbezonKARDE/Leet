class Solution {
    public boolean judgeCircle(String moves) {
        int[] ch = new int[26];
        for (char move : moves.toCharArray()){
            ch[move - 'A']++;
        }
        return ch['U' - 'A'] == ch['D' - 'A'] && 
                ch['L' - 'A'] == ch['R' - 'A'];
    }
}
