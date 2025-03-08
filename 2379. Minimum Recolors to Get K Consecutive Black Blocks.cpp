class Solution {
public:
    int minimumRecolors(string blocks, int k) {
        int low = 0 , high = 0 , size = blocks.size() ; 
        int white = 0  , res = INT_MAX;
        while(high  <size){
            white += (blocks[high] == 'W');
            while(high - low + 1 >= k){
                res = min(res , white);
                white -= (blocks[low++] == 'W');
            }
            high++;
        }
        return res;
    }
};
