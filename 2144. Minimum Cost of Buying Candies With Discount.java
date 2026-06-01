class Solution {
    public int minimumCost(int[] costs) {
        int[] candies = new int[101];
        for (int c: costs) candies[c]++;
        int bought = 0;
        int cost = 0;
        for (int i = 100; i>0; i--) {
            if (candies[i] > 2-bought) {
                candies[i] -= (3-bought);
                cost += (2-bought)*i;
                cost += 2*i*(candies[i]/3);
                bought = candies[i]%3;
                cost += i*bought;
            } else {
                bought += candies[i];
                cost += i*candies[i];
            }
        }
        return cost;
    }
}
