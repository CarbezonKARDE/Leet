class Solution {
public:
    int longestBalanced(string s) {
        int n = s.size();
        if (n == 0) return 0;
        int a = 0, b = 0, c = 0;
        int lastA = -1, lastB = -1, lastC = -1;
        int maxLen = 0;
        int offset = n;
        int sz = 2 * n + 3;
        vector<vector<int>> ab(sz), bc(sz), ac(sz);
        vector<int> headAB(sz, 0), headBC(sz, 0), headAC(sz, 0);
        ab[offset].push_back(-1);
        bc[offset].push_back(-1);
        ac[offset].push_back(-1);
        using ll = long long;
        unordered_map<ll, int> firstPair;
        firstPair.reserve(2 * n + 3);
        ll key0 = (static_cast<ll>(offset) << 32) ^ static_cast<unsigned int>(offset);
        firstPair[key0] = -1;

        for (int i = 0; i < n; ++i) {
            char ch = s[i];
            if (ch == 'a') {
                ++a;
                lastA = i;
            } else if (ch == 'b') {
                ++b;
                lastB = i;
            } else {
                ++c;
                lastC = i;
            }
            int abDiff = a - b;
            int bcDiff = b - c;
            int acDiff = a - c;
            int j1 = max(lastB, lastC);
            if (i - j1 > maxLen) maxLen = i - j1;
            int j2 = max(lastA, lastC);
            if (i - j2 > maxLen) maxLen = i - j2;
            int j3 = max(lastA, lastB);
            if (i - j3 > maxLen) maxLen = i - j3;
            int abIdx = abDiff + offset;
            auto &vAB = ab[abIdx];
            int &hAB = headAB[abIdx];
            while (hAB < (int)vAB.size() && vAB[hAB] < lastC) ++hAB;
            if (hAB < (int)vAB.size()) {
                int len = i - vAB[hAB];
                if (len > maxLen) maxLen = len;
            }
            int bcIdx = bcDiff + offset;
            auto &vBC = bc[bcIdx];
            int &hBC = headBC[bcIdx];
            while (hBC < (int)vBC.size() && vBC[hBC] < lastA) ++hBC;
            if (hBC < (int)vBC.size()) {
                int len = i - vBC[hBC];
                if (len > maxLen) maxLen = len;
            }
            int acIdx = acDiff + offset;
            auto &vAC = ac[acIdx];
            int &hAC = headAC[acIdx];
            while (hAC < (int)vAC.size() && vAC[hAC] < lastB) ++hAC;
            if (hAC < (int)vAC.size()) {
                int len = i - vAC[hAC];
                if (len > maxLen) maxLen = len;
            }
            ll key = (static_cast<ll>(abDiff + offset) << 32) ^
                     static_cast<unsigned int>(bcDiff + offset);
            auto it = firstPair.find(key);
            if (it != firstPair.end()) {
                int len = i - it->second;
                if (len > maxLen) maxLen = len;
            } else {
                firstPair[key] = i;
            }
            vAB.push_back(i);
            vBC.push_back(i);
            vAC.push_back(i);
        }
        return maxLen;
    }
};
