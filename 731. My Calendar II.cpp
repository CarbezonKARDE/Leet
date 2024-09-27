class MyCalendarTwo {
    using int2=pair<int, int>;
    vector<int2> S1, S2;
public:
    MyCalendarTwo() {
        S1.reserve(1000);
        S2.reserve(1000);
    }
    bool book(int start, int end) {
        for(auto& [s, t]: S2){
            if (max(s, start)<min(t, end))
                return 0;
        }
        for (auto& [s, t]: S1){
            int s0=max(s, start), t0=min(t, end);
            if  (s0 < t0)
                S2.emplace_back(s0, t0);
        }
        S1.emplace_back(start, end);
        return 1;
    }
};
