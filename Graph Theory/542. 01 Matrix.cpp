class Solution {
public:
    vector<vector<int>> updateMatrix(vector<vector<int>>& mat) {
        int n=mat.size();
        int m=mat[0].size();
        vector<vector<int>> mat1(n, vector<int>(m, 100000));
        queue<pair<int, int>> q;
        int k=0;
        for(int i=0;i<n;i++){
            for(int j=0;j<m;j++){
                if(mat[i][j]==0){
                    mat1[i][j]=0;
                    q.push({i,j});
                }
                
            }
        }
        while(!q.empty()){
            int i=q.front().first;
            int j=q.front().second;
            q.pop();
            if(i+1<n && mat1[i+1][j]>mat1[i][j]+1){
                mat1[i+1][j]=mat1[i][j]+1;
                q.push({i+1,j});
            }
            if(i-1>=0 && mat1[i-1][j]>mat1[i][j]+1){
                mat1[i-1][j]=mat1[i][j]+1;
                q.push({i-1,j});
            }
            if(j+1<m && mat1[i][j+1]>mat1[i][j]+1){
                mat1[i][j+1]=mat1[i][j]+1;
                q.push({i,j+1});
            }
            if(j-1>=0 && mat1[i][j-1]>mat1[i][j]+1){
                mat1[i][j-1]=mat1[i][j]+1;
                q.push({i,j-1});
            }
        }
        return mat1;
    }
};
