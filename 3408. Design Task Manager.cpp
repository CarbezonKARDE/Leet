class TaskManager {
public:
    unordered_map<int, pair<int, int>> mp;
    priority_queue<pair<int, int>> pq;
    TaskManager(vector<vector<int>>& tasks) {
        for (int i = 0; i < tasks.size(); i++) {
            int u = tasks[i][0];
            int t = tasks[i][1];
            int p = tasks[i][2];
            mp[t] = {u, p};
            pq.push({p, t});
        }
    }
    void add(int userId, int taskId, int priority) {
        mp[taskId] = {userId, priority};
        pq.push({priority, taskId});
    }
    void edit(int taskId, int newPriority) {
        mp[taskId].second = newPriority;
        pq.push({newPriority, taskId});
    }
    void rmv(int taskId) { mp.erase(taskId); }
    int execTop() {
        while (!pq.empty()) {
            int p = pq.top().first;
            int t = pq.top().second;
            pq.pop();
            if (mp.find(t) != mp.end() && mp[t].second == p) {
                int user = mp[t].first;
                mp.erase(t);
                return user;
            }
        }
        return -1;
    }
};
