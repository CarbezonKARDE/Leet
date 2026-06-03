class Solution {
public:
    static int earliestFinishTime(vector<int>& landStartTime, vector<int>& landDuration, vector<int>& waterStartTime, vector<int>& waterDuration) {
        const int n=landStartTime.size(), m=waterStartTime.size();
        int minLandEnd=1e6, minWaterEnd=1e6;
        for(int i=0; i<n; i++) 
            minLandEnd=min(minLandEnd, landStartTime[i]+landDuration[i]);
        int ans=1e9;
        for(int i=0; i<m; i++){
            minWaterEnd=min(minWaterEnd, waterStartTime[i]+waterDuration[i]);
            ans=min(ans,  max(minLandEnd, waterStartTime[i])+waterDuration[i]);
        }
        for(int i=0; i<n; i++){
            ans=min(ans, max(minWaterEnd, landStartTime[i])+landDuration[i]);
        }
        return ans;
    }
};
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
