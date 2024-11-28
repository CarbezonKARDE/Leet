class Solution {
public:
    void solve(int row,int col,int &count,vector<vector<int>>& v,
    vector<vector<bool>>& temp)
    {
        if(v[row][col]!=1)
            return;
        
        if(row+1<v.size() && !temp[row+1][col])
        {
            temp[row+1][col]=true;
            if(v[row+1][col]==1)
                count++;
            solve(row+1,col,count,v,temp);
        }
        if(row-1>=0 && !temp[row-1][col])
        {
            temp[row-1][col]=true;
            if(v[row-1][col]==1)
                count++;
            solve(row-1,col,count,v,temp);
        }
        if(col+1<v[row].size() && !temp[row][col+1])
        {
            temp[row][col+1]=true;
            if(v[row][col+1]==1)
                count++;
            solve(row,col+1,count,v,temp);
        }
        if(col-1>=0 && !temp[row][col-1])
        {
            temp[row][col-1]=true;
            if(v[row][col-1]==1)
                count++;
            solve(row,col-1,count,v,temp);
        }
    }
    int maxAreaOfIsland(vector<vector<int>>& v) {
        vector<vector<bool>>temp(v.size(),vector<bool>(v[0].size(),false));
        int ans=0;
        for(int i=0;i<v.size();i++)
        {
            for(int j=0;j<v[i].size();j++)
            {
                if(v[i][j]==1 && !temp[i][j])
                {
                    int count=1;
                    temp[i][j]=true;
                    solve(i,j,count,v,temp);
                    ans=max(ans,count);
                }
            }
        }
        return ans;
    }
};
