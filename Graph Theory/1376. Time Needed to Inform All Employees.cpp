class Solution {
public:
    int numOfMinutes(int n, int headID, vector<int>& manager, vector<int>& informTime) {
        vector<vector<int>>v(n);
        for(int i=0;i<n;i++){
           if(manager[i]!= -1){
               v[manager[i]].push_back(i);
           }
        }
        queue<pair<int,int>>dq;
        int maxi=0;
        dq.push({headID,0});
        while(!dq.empty()){
           int ind = dq.front().first;
           int cnt = dq.front().second; 
           maxi=max(cnt,maxi);
           dq.pop();
           for(auto it : v[ind]){
                dq.push({it,cnt+informTime[ind]});
            }
        }
        return maxi;
    }
};
