class Solution {
public:
    bool uniformArray(auto& A) {
        uint32_t x[2] = {-1u, -1u};
        for (uint32_t a : A)
            x[a & 1] = min(x[a & 1], a);
        return x[1] < x[0] | x[1] == -1u;
    }
};
