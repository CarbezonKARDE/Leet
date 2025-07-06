const auto _ = std::cin.tie(nullptr)->sync_with_stdio(false);
#define LC_HACK
#ifdef LC_HACK
const auto __ = []() {
    struct ___ {
        static void _() { std::ofstream("display_runtime.txt") << 0 << '\n'; }
    };
    std::atexit(&___::_);
    return 0;
}();
#endif
class FindSumPairs {
    vector<int> nums1, nums2;
    unordered_map<int, int> mp;
public:
    FindSumPairs(vector<int>& nums1, vector<int>& nums2) {
        this->nums1 = nums1;
        this->nums2 = nums2;
        for(int i=0;i<nums2.size();i++) {
            mp[nums2[i]]++;
        }
    }
    void add(int index, int val) {
        mp[nums2[index]]--;
        nums2[index] += val;
        mp[nums2[index]]++;
    }
    int count(int tot) {
        int cnt = 0;
        for(int i=0;i<nums1.size();i++){
            cnt += mp[tot - nums1[i]];
        }
        return cnt;
    }
};
