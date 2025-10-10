class Solution {
    public int[] successfulPairs(int[] spells, int[] potions, long success) {
        Arrays.sort(potions);
        int n= spells.length;
        int[] ans= new int[n];
        for(int i=0; i<n; i++){
            int l=0, h= potions.length-1;
            int cur= spells[i];
            int ind=potions.length;
            while(l<=h){
                int mid=l+(h-l)/2;
                if((long)cur*potions[mid]>=success){
                    ind= mid;
                    h= mid-1;
                }else{
                    l=mid+1;
                }
            }
            ans[i]= potions.length-ind;
        }
        return ans;
    }
}
