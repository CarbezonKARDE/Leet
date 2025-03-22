class Solution {
public:
int findPar(int node, vector<int>&par)
{
    if(node==par[node])
    {
        return node;
    }
    return par[node]= findPar(par[node], par);
}
    int countCompleteComponents(int n, vector<vector<int>>& edges) {
        vector<int>count(n,1),par(n),edge(n,0), rank(n,0);
        for(int i=0;i<n;i++)
        {
            par[i]=i;
        }
        int par1,par2;
        for(int i=0;i<edges.size();i++)
        {
            par1=findPar(edges[i][0], par);
            par2=findPar(edges[i][1], par);

            if(par1==par2)
            {
                edge[par1]++;
            }
            else
            {
                if(rank[par1]==rank[par2])
                {
                par[par1]=par2;
                count[par2]+=count[par1];
                edge[par2]+=edge[par1]+1;
                rank[par2]++;
                }
                else if(rank[par1]>rank[par2])
                {
                par[par2]=par1;
                count[par1]+=count[par2];
                edge[par1]+=edge[par2]+1; 
                }
                else
                {
                par[par1]=par2;
                count[par2]+=count[par1];
                edge[par2]+=edge[par1]+1;                    
                }

            }
        }
        int res=0;
        for(int i=0;i<n;i++)
        {
            if(i==par[i] && edge[i]==(count[i]*(count[i]-1))/2)
            {
                res++;
            }
        }
        return res;
    }
};
