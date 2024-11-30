class Solution {
private:
    vector<int> idMap;
    int getRootParent(int x) {
        if (idMap[x] != x) {
            idMap[x] = getRootParent(idMap[x]);
        }
        return idMap[x];
    }
public:
    vector<vector<string>> accountsMerge(vector<vector<string>>& accounts) {
        int n = accounts.size();
        idMap.resize(n);
        for (int i = 0; i < n; ++i) {
            idMap[i] = i;
        }
        unordered_map<string, int> emailToIdMap;
        for (int i = 0; i < n; ++i) {
            const auto& account = accounts[i];
            for (int j = 1; j < account.size(); ++j) {
                const string& email = account[j];
                auto it = emailToIdMap.find(email);
                if (it != emailToIdMap.end()) {
                    idMap[getRootParent(i)] = getRootParent(it->second);
                } else {
                    emailToIdMap[email] = i;
                }
            }
        }
        unordered_map<int, vector<string>> mergedMap;
        for (const auto& entry : emailToIdMap) {
            int id = getRootParent(entry.second);
            mergedMap[id].push_back(entry.first);
        }
        vector<vector<string>> mergedAccounts;
        for (auto& entry : mergedMap) {
            vector<string>& emails = entry.second;
            sort(emails.begin(), emails.end());
            emails.insert(emails.begin(), accounts[entry.first][0]);
            mergedAccounts.push_back(move(emails));
        }
        return mergedAccounts;
    }
};
