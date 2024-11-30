typedef vector<int> vi;
typedef vector<bool> vb;
typedef vector<vb> vvb;
class Solution {
public:
    int maximalNetworkRank(int n, vector<vector<int>>& roads) {
        vi ind(n, 0);
        vvb flag(n, vb(n, false));
        for(auto& r: roads){
            ind[r[0]]++;
            ind[r[1]]++;
            flag[r[0]][r[1]] = true;
            flag[r[1]][r[0]] = true;
        }
        int ans = 0, tmp;
        for(int i = 0; i < n; i++)
            for(int j = i + 1; j < n; j++){
                if(ind[i] + ind[j] > ans){
                    if(flag[i][j])
                        ans = ind[i] + ind[j] - 1;
                    else ans = ind[i] + ind[j];
                }
            }    
        return ans;
    }
};
