class DSU{
    vector<int>size, parent;
    public:
        DSU(int n){
            size.resize(n, 1);
            parent.resize(n);
            for(int i=0;i<n;i++)
            parent[i] = i;
        }
        int findPar(int node){
            if(node == parent[node])
            return node;
            return parent[node] = findPar(parent[node]);
        }
        void Union(int u, int v){
            int u_par = findPar(u);
            int v_par = findPar(v);
            if(u_par == v_par)
            return;
            if(u_par <= v_par )
            {
                parent[v_par] = u_par;
            }
            else{
                parent[u_par] = v_par;
            }
        }
};
class Solution {
public:
    string smallestEquivalentString(string s1, string s2, string baseStr) {
        DSU ds(26);
        for(int i=0;i<s1.size();i++)
        ds.Union(s1[i] - 'a', s2[i] - 'a');
        string ans = "";
        for(int i=0;i<baseStr.size();i++)
        ans+= ds.findPar(baseStr[i] - 'a') + 'a';
        return ans;
    }
};
