class Solution {
public:
    int minOperations(vector<int>& nums, int k) {
        unordered_map<int, int>mp;
        int minVal = INT_MAX;
        for(auto n : nums){
            mp[n] = 1;
            minVal = min(minVal, n);
        }
        if(minVal < k)
            return -1;
        int ans = mp.size();
        if(mp[k]) ans--;
        return ans;
    }
};
