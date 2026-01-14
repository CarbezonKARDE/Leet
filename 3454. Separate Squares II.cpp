#include <vector>
#include <algorithm>
using namespace std;
class Solution {
public:
    double separateSquares(vector<vector<int>>& sq) {
        vector<int> X;
        X.reserve(sq.size() * 2);
        struct Event {
            int y, type, xl, xr;
            bool operator<(const Event& other) const { return y < other.y; }
        };
        vector<Event> events;
        events.reserve(sq.size() * 2);
        for(const auto& s : sq) {
            X.push_back(s[0]);
            X.push_back(s[0] + s[2]);
            events.push_back({s[1], 1, s[0], s[0] + s[2]});
            events.push_back({s[1] + s[2], -1, s[0], s[0] + s[2]});
        }
        sort(X.begin(), X.end());
        X.erase(unique(X.begin(), X.end()), X.end());
        for(auto& e : events) {
            e.xl = lower_bound(X.begin(), X.end(), e.xl) - X.begin();
            e.xr = lower_bound(X.begin(), X.end(), e.xr) - X.begin();
        }
        sort(events.begin(), events.end());
        int n = X.size();
        vector<int> count(4 * n, 0);
        vector<long long> len(4 * n, 0);
        auto update = [&](auto&& self, int node, int start, int end, int l, int r, int val) -> void {
            if(l >= r) return;
            if(l == start && r == end) {
                count[node] += val;
            } else {
                int mid = start + (end - start) / 2;
                if(l < mid) self(self, 2 * node, start, mid, l, min(r, mid), val);
                if(r > mid) self(self, 2 * node + 1, mid, end, max(l, mid), r, val);
            }
            if(count[node] > 0) {
                len[node] = (long long)X[end] - X[start];
            } else if(start + 1 == end) {
                len[node] = 0;
            } else {
                len[node] = len[2 * node] + len[2 * node + 1];
            }
        };
        long long total_area = 0;
        vector<long long> h_dy;
        vector<long long> h_width;
        h_dy.reserve(events.size());
        h_width.reserve(events.size());
        int prev_y = events[0].y;
        for(const auto& e : events) {
            if(e.y > prev_y) {
                long long width = len[1];
                long long dy = e.y - prev_y;
                total_area += width * dy;
                h_dy.push_back(dy);
                h_width.push_back(width);
            }
            update(update, 1, 0, n - 1, e.xl, e.xr, e.type);
            prev_y = e.y;
        }
        double target = (double)total_area / 2.0;
        double curr_area = 0;
        double curr_y = events[0].y;
        for(size_t i = 0; i < h_dy.size(); ++i) {
            double dy = h_dy[i];
            double w = h_width[i];
            double area = dy * w;
            if(curr_area + area >= target) {
                return curr_y + (target - curr_area) / w;
            }
            curr_area += area;
            curr_y += dy;
        }
        return curr_y;
    }
};
