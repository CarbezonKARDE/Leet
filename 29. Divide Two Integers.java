class Solution {
    public int divide(int dividend, int divisor) {
        if (dividend == Integer.MIN_VALUE && divisor == -1) {
            return Integer.MAX_VALUE; 
        }
        int sign = (dividend > 0) == (divisor > 0) ? 1 : -1;
        long absDividend = Math.abs((long) dividend);
        long absDivisor = Math.abs((long) divisor);
        long quotient = 0;
        while (absDividend >= absDivisor) {
            long tempDivisor = absDivisor;
            long multiple = 1;
            while (absDividend >= (tempDivisor << 1)) {
                tempDivisor <<= 1;
                multiple <<= 1;
            }
            absDividend -= tempDivisor;
            quotient += multiple;
        }
        return (int) (sign * quotient);
    }
}
