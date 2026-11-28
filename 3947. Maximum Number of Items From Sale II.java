import java.util.*;
class Solution {
    public int maximumSaleItems(int[][] items, int budget) {
        int n = items.length;
        int[] cnt = new int[n + 1];
        for (int[] it : items) cnt[it[0]]++;
        int[] mult = new int[n + 1];
        for (int f = 1; f <= n; f++)
            for (int k = f; k <= n; k += f) mult[f] += cnt[k];
        long[] keys = new long[n];
        long minPrice = Long.MAX_VALUE;
        for (int i = 0; i < n; i++) {
            long price = items[i][1];
            minPrice = Math.min(minPrice, price);
            keys[i] = (price << 17) | (mult[items[i][0]] - 1);
        }
        Arrays.sort(keys);
        long rem = budget, taken = 0;
        long mask = (1L << 17) - 1;
        for (long key : keys) {
            long price = key >> 17, gifts = key & mask;
            if (price >= 2 * minPrice) break;
            long t = Math.min(gifts, rem / price);
            taken += t;
            rem -= t * price;
            if (t < gifts) break;
        }
        return (int) (2 * taken + rem / minPrice);
    }
}
