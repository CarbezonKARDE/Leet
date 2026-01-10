class Solution {
public:
    int minimumDeleteSum(std::string s1, std::string s2) {
        const int n = static_cast<int>(s1.size());
        const int m = static_cast<int>(s2.size());
        int sum1 = 0, sum2 = 0;
        for (char c : s1) sum1 += static_cast<unsigned char>(c);
        for (char c : s2) sum2 += static_cast<unsigned char>(c);
        std::vector<int> dp(m + 1, 0);
        for (int i = 1; i <= n; i++) {
            int prevDiag = 0; 
            for (int j = 1; j <= m; j++) {
                int savedUp = dp[j]; 
                if (s1[i - 1] == s2[j - 1]) {
                    dp[j] = prevDiag + static_cast<unsigned char>(s1[i - 1]);
                } else {
                    dp[j] = std::max(dp[j], dp[j - 1]); 
                }
                prevDiag = savedUp; 
            }
        }
        const int keep = dp[m];
        return sum1 + sum2 - 2 * keep;
    }
};
