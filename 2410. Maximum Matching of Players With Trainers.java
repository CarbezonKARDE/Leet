class Solution {
    public int matchPlayersAndTrainers(int[] players, int[] trainers) {
        int m = players.length;
        int n = trainers.length;
        Thread t1 = new Thread(() -> Arrays.sort(players));
        Thread t2 = new Thread(() -> Arrays.sort(trainers));
        t1.start();
        t2.start();
        try {
            t1.join();
            t2.join();
        } catch (Exception e) {
        }
        int left = 0;
        int right = 0;
        int count = 0;
        while (left < m && right < n) {
            if (trainers[right] >= players[left]) {
                count++;
                left++;
            }
            right++;
        }
        return count;
    }
}
