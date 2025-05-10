class Solution {
public:
    static long long minSum(vector<int>& nums1, vector<int>& nums2) {
        long long sum1=0, sum2=0;
        int zero1=0, zero2=0;
        for(int x: nums1){
            sum1+=x;
            zero1+=x==0;
        }
        for(int x: nums2){
            sum2+=x;
            zero2+=x==0;
        }
        if ((zero1==0 && sum1<sum2+zero2)||(zero2==0 && sum2<sum1+zero1))
            return -1;
        return max(sum1+zero1, sum2+zero2);
    }
};
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
