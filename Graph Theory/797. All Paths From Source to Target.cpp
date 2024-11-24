class Solution {
public:
    void allPath(vector<vector<int>>& adj,vector<bool> &visited,int start,int destination,vector<int> &path,vector<vector<int>> &allPaths){
        if(start == destination){
            path.push_back(start);
            allPaths.push_back(path);
            path.pop_back();
            return; 
        }
        path.push_back(start);
        visited[start]=true;
        for(auto i:adj[start]){
            if(!visited[i]){
                allPath(adj,visited,i,destination,path,allPaths);
            }
        }
        path.pop_back();
        visited[start]=false;
    }
    vector<vector<int>> allPathsSourceTarget(vector<vector<int>>& graph) {
        int size=graph.size();
        vector<bool> visited(size,false);
        int start=0;
        int destination=size-1;
        vector<int> path;
        vector<vector<int>> allPaths;
        allPath(graph,visited,start,destination,path,allPaths);
        return allPaths;
    }
};
