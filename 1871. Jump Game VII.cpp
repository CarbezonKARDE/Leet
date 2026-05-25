int q[100000];
int front, back;
class Solution {
public:
    static bool canReach(string& s, int minJ, int maxJ) {
        const int n=s.size();
        if (s[n-1]=='1') return 0;
        front=back=0;
        q[back++]=0;
        int i=0;
        for(int far=0; front<back; far=max(far, i+maxJ)) {
            i=q[front++];
            int j0=max(far+1, i+minJ), jM=min(i+maxJ, n-1);
            for (int j=j0; j<=jM; j++) {
                if (s[j]=='0') {
                    if (j==n-1) return 1;
                    q[back++]=j;
                }
            }
        }
        return 0;
    }
};
