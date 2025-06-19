class Solution {
    public int partitionArray(int[] nums, int k) {
        int min=Integer.MAX_VALUE;
        int max=Integer.MIN_VALUE;
        for(int num:nums){
            if(num<min) min=num;
            if(num>max) max=num;
        }
        if(max-min<=k) return 1;
        int[] count=new int[max-min+1];
        for(int x:nums){
            ++count[x-min];
        }
        int ans=1;
        int x=k+1;
        max=max-min;
        while(x<=max){
            while(x<=max && count[x]==0){
                x++;
            }
            if(x<=max){
                ans++;
                x+=k+1;
            }
        }
        return ans;
    }
}
