class Solution {
public:
    int maxActiveSectionsAfterTrade(string s) {
        int n = s.size();
        int ones_count = 0;
        int min_one_block = n + 1;
        int max_zero_block = 0;
        int max_adj = 0;
        int prev_one_block_len = 0;
        int prev_zero_block_len = 0;
        for (int i = 0; i < n; ) {
            int j;
            if (s[i] == '0') {
                j = s.find('1', i + 1);
                if (j == -1) {
                    j = n;
                }
                int block_len = j - i;
                max_zero_block = max(max_zero_block, block_len);
                if (prev_zero_block_len > 0) {
                    max_adj = max(max_adj, prev_zero_block_len + block_len);
                    if (prev_one_block_len > 0) {
                        min_one_block = min(min_one_block, prev_one_block_len);
                    }
                }
                prev_zero_block_len = block_len;
            } else {
                j = s.find('0', i + 1);
                if (j == -1) {
                    j = n;
                }
                int block_len = j - i;
                ones_count += block_len;                
                prev_one_block_len = block_len;
            }
            i = j;
        }
        int ans = ones_count;
        ans = max(ans, ones_count + max_adj);
        if (min_one_block <= n && max_zero_block > 0) {
            ans = max(ans, ones_count - min_one_block + max_zero_block);
        }
        return ans;
    }
};
