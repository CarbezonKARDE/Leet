class Solution {
    public int countConsistentStrings(String allowed, String[] words) {
        boolean[] alpha = new boolean[26];
        int length= allowed.length();
        for(int i=0;i<length;i++){
            alpha[allowed.charAt(i)-'a']=true;
        }
        int count =words.length;
        for(int i=0;i<words.length;i++){
            length=words[i].length();
            for(int j=0;j<length;j++){
            if(!alpha[words[i].charAt(j)-'a']){
                count--;
                break;
            }
        }
        }
        return count;
    }
}
