const int N=1<<11;
uint8_t seen3[N];
uint64_t seen2[32];
class Solution {
public:
    static int uniqueXorTriplets(vector<int>& nums) {
        const int n=nums.size();
        if (n==1) return 1;
        unsigned M=*max_element(nums.begin(), nums.end());
        M=(bit_floor(M)<<1)-1;
        memset(seen3, 0, M+1);
        int qM=M>>6;
        memset(seen2, 0, (qM+1)*sizeof(uint64_t));
        int cnt=0;
        for(int i=0; i<n; i++){
            const int x=nums[i];
            cnt+=(!seen3[x]);
            seen3[x]=1;
            for(int j=i+1; j<n; j++){
                const int y=x^nums[j];
                const int q=y>>6, r=y&63;
                seen2[q]|=(1ULL<<r);
            }
            for(int q=0; q<=qM; q++){
                uint64_t B=seen2[q];
                for( ; B; B&=(B-1)){
                    const int r=__builtin_ctzll(B);
                    const int y=(q<<6)+r;
                    cnt+=(!seen3[x^y]);
                    seen3[x^y]=1;
                }
            }
        }
        return cnt;
    }
};
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
