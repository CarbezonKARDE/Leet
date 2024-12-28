class Solution {
    vector<int> prefix_sum;
    int max_sum;
    int mem[20001][3];
    vector<int> max_sum_path;
    int findMaxSum(vector<int>& nums,int pos,int count,int& k){
        if(count==3)              return 0;
        if(pos>nums.size()-k)     return 0;
        if(mem[pos][count]!=-1)   return mem[pos][count];        
        int dont_start = findMaxSum(nums,pos+1,count,k);
        int start_here = findMaxSum(nums,pos+k,count+1,k)
                         + prefix_sum[pos+k]-prefix_sum[pos];

        return mem[pos][count] = max(dont_start,start_here);
    }
    void findMaxSumPath(vector<int>& nums,int pos,int count,int& k,vector<int>& path){
        if(count==3)             return;
        if(pos>nums.size()-k)    return;
        int dont_start = findMaxSum(nums,pos+1,count,k);
        int start_here = findMaxSum(nums,pos+k,count+1,k)
                         + prefix_sum[pos+k]-prefix_sum[pos];        
        if(start_here >= dont_start){
            path.push_back(pos);
            findMaxSumPath(nums,pos+k,count+1,k,path);
        }else{
            findMaxSumPath(nums,pos+1,count,k,path);
        }
    }
public:
    vector<int> maxSumOfThreeSubarrays(vector<int>& nums, int k) {
        int n = nums.size();
        memset(mem,-1,sizeof(mem));
        prefix_sum = vector<int>(n+1,0);
        for(int i=0;i<n;++i)
            prefix_sum[i+1] = prefix_sum[i]+nums[i];
        max_sum = findMaxSum(nums,0,0,k);
        vector<int> path;
        findMaxSumPath(nums,0,0,k,path);
        return path;
    }
};
