class Solution {
    public int[] pathsWithMaxScore(List<String> board) {
        int MOD = 1_000_000_007;
        int n = board.size();
        int[] score = new int[n + 1];
        int[] ways = new int[n + 1];
        Arrays.fill(score, -1);
        for (int r = n - 1; r >= 0; r--) {
            int[] newScore = new int[n + 1];
            int[] newWays = new int[n + 1];
            Arrays.fill(newScore, -1);
            for (int c = n - 1; c >= 0; c--) {
                char ch = board.get(r).charAt(c);
                if (ch == 'X') 
                    continue;
                if (ch == 'S') {
                    newScore[c] = 0;
                    newWays[c] = 1;
                    continue;
                }
                int best = Math.max(score[c], Math.max(newScore[c + 1], score[c + 1]));
                if (best == -1) 
                    continue;
                long cnt = 0;
                if (score[c] == best) 
                    cnt += ways[c];
                if (newScore[c + 1] == best) 
                    cnt += newWays[c + 1];
                if (score[c + 1] == best) 
                    cnt += ways[c + 1];
                int val = ch == 'E' ? 0 : ch - '0';
                newScore[c] = best + val;
                newWays[c] = (int)(cnt % MOD);
            }
            score = newScore;
            ways = newWays;
        }
        if (score[0] == -1) 
            return new int[]{0, 0};
        return new int[]{score[0], ways[0]};
    }
}
