class Solution {
public:
    bool canVisitAllRooms(vector<vector<int>>& rooms) {
        int n = rooms.size();
        vector<bool> visited(n, false);
        queue<int>Queue;
        Queue.push(0);
        visited[0] = true;
        while(!Queue.empty()){
            vector<int> room = rooms[Queue.front()];
            Queue.pop();
            for(int key : room){
                if(!visited[key]){
                    visited[key] = true;
                    Queue.push(key);
                }
            }
        }
        bool ans = true;
        for(int i=0 ; i<n ; i++){
            ans = ans&visited[i];
        }
        if(ans) return true;
        else return false; 
    }
};
