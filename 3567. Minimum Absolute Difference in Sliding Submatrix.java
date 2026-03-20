class Solution {
    public int[][] minAbsDiff(int[][] grid, int k) {
        int [][]res = new int[grid.length-k+1][grid[0].length-k+1];
        for(int i=k-1; i<grid.length; i++) {
            for(int j=k-1;j<grid[0].length;j++) {
                int []temp = new int[k*k];
                for(int ii=i-k+1;ii<=i;ii++) {
                    for(int jj=j-k+1;jj<=j;jj++) {
                        temp[(ii-(i-k+1))*k + (jj-(j-k+1))] = grid[ii][jj];
                    }
                }
                Arrays.sort(temp);
                res[i-(k-1)][j-(k-1)] = temp[temp.length-1] - temp[0];
                for(int kk=1; kk<temp.length; kk++) {
                    if(temp[kk]==temp[kk-1]) {
                        continue;
                    }
                    res[i-(k-1)][j-(k-1)] = Math.min(res[i-(k-1)][j-(k-1)], temp[kk]-temp[kk-1]);
                }
            }
        }
        return res;
    }
}
