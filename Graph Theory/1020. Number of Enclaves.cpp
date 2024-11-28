class Solution {
public:
    int count=0;
    void dfs(vector<vector<int>>& A, int i, int j) {
        if (!(i>=0 && j>=0 && i<A.size() && j<A[0].size()))
        { 
            return;
        }
        if (A[i][j]==1) 
        {
            A[i][j]=-1;
        } else return;
        dfs(A,i-1,j);
        dfs(A,i+1,j);
        dfs(A,i,j-1);
        dfs(A,i,j+1);
    }
    int numEnclaves(vector<vector<int>>& A) {
        for (int i=0; i<A.size(); i++)
        {
            for (int j=0; j<A[i].size(); j++)
            {
                if ((i==0 || j==0 || i==A.size()-1 || j==A[i].size()-1) && A[i][j]==1)
                {
                    dfs(A, i, j);
                }
            }
        }
        for (int i=0; i<A.size(); i++)
        {
            for (int j=0; j<A[i].size(); j++)
            {
                if (A[i][j]==1)
                {
                    count++;
                }
            }
        }
        return count;
    }
};
