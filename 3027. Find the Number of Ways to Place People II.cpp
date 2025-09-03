#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    int numberOfPairs(vector<vector<int>>& points) {
        sort(points.begin(), points.end(), [](const vector<int>& a, const vector<int>& b) {
            if (a[0] == b[0]) return a[1] > b[1];
            return a[0] < b[0];
        });
        int pairCount = 0;
        int n = points.size();
        for (int i = 0; i < n; i++) {
            int upperY = points[i][1];
            int lowerYLimit = INT_MIN;
            for (int j = i + 1; j < n; j++) {
                int currentY = points[j][1];
                if (currentY <= upperY && currentY > lowerYLimit) {
                    pairCount++;
                    lowerYLimit = currentY;
                    if (currentY == upperY) break;
                }
            }
        }
        return pairCount;
    }
};

