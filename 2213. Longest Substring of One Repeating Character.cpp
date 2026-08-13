class Solution {
public:
    vector<int> longestRepeating(string s, string chars, vector<int>& indices) {
        int n = s.size();
        set<int> boundaries;
        set<int> lengths;
        vector<int> count(n+1);
        auto addlen = [&](int len) {
            if (++count[len] == 1)
                lengths.insert(len);
        };
        auto removelen = [&](int len) {
            if (--count[len] == 0)
                lengths.erase(len);
        };
        boundaries.insert(0);
        boundaries.insert(n);
        int prev = 0;
        for (int i = 1; i < n; ++i) {
            if (s[i - 1] != s[i]) {
                boundaries.insert(i);
                addlen(i - prev);
                prev = i;
            }
        }
        addlen(n - prev);
        auto update = [&](int i, bool set) {
            if (set) {
                auto it = boundaries.lower_bound(i);
                int next = *it;
                int prev = *std::prev(it);
                removelen(next - prev);
                addlen(i - prev);
                addlen(next - i);
                boundaries.insert(i);
            } else {
                auto it = boundaries.find(i);
                int prev = *std::prev(it);
                int next = *std::next(it);
                removelen(i - prev);
                removelen(next - i);
                addlen(next - prev);
                boundaries.erase(it);
            }
        };
        vector<int> result(indices.size());
        for (int i = 0; i < indices.size(); ++i) {
            int idx = indices[i];
            char c = chars[i];
            if (idx > 0) {
                bool old = s[idx - 1] != s[idx];
                bool now = s[idx - 1] != c;
                if (old != now)
                    update(idx, now);
            }
            if (idx + 1 < n) {
                bool old = s[idx] != s[idx + 1];
                bool now = c != s[idx + 1];
                if (old != now)
                    update(idx + 1, now);
            }
            s[idx] = c;
            result[i] = *lengths.rbegin();
        }
        return result;
    }
};
