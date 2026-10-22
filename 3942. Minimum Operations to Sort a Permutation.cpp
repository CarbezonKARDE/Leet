class Solution {
public:
    int minOperations(vector<int>& nums) {
        int n = nums.size(), shift = ranges::find(nums, 0) - nums.begin();
        auto ring = [&](int k) { return nums[(shift + k) % n]; };
        auto all = [&](auto holds) { return ranges::all_of(views::iota(0, n), holds); };
        if (all([&](int k) { return ring(k) == k; }))
            return min(shift, n - shift + 2);
        if (all([&](int k) { return ring(k) == (n - k) % n; }))
            return min(shift + 2, n - shift);
        return -1;
    }
};
