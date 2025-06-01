class Solution {
public:
    long long combCount(int sum) {
        if (sum < 0) return 0;
        return 1LL * (sum + 2) * (sum + 1) / 2;
    }
    long long distributeCandies(int n, int limit) {
        return combCount(n)
             - 3 * combCount(n - (limit + 1))
             + 3 * combCount(n - 2 * (limit + 1))
             - combCount(n - 3 * (limit + 1));
    }
};
