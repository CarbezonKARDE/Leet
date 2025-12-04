class Solution {
    public int countCollisions(String directions) {
        int n = directions.length(), ans = 0, l = 0, r = n - 1; 
        while(l < n && directions.charAt(l) == 'L') l++;
        while(r >= l && directions.charAt(r) == 'R') r--;
        for(int i = l ; i <= r; i++) {
            if(directions.charAt(i) != 'S') ans++;
        }
        return ans;
    }
}
