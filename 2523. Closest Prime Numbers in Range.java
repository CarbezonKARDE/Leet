class Solution {
    public int[] closestPrimes(int left, int right) {
        List<Integer> primeNumbers = new ArrayList<>();
        for (int candidate = left; candidate <= right; candidate++) {
            if (candidate % 2 == 0 && candidate > 2) {
                continue;
            }
            if (isPrime(candidate)) {
                if (
                    !primeNumbers.isEmpty() &&
                    candidate <= primeNumbers.get(primeNumbers.size() - 1) + 2
                ) {
                    return new int[] {
                        primeNumbers.get(primeNumbers.size() - 1),
                        candidate,
                    };
                }
                primeNumbers.add(candidate);
            }
        }
        if (primeNumbers.size() < 2) {
            return new int[] { -1, -1 };
        }
        int[] closestPair = new int[] { -1, -1 };
        int minDifference = 1000000;
        for (int index = 1; index < primeNumbers.size(); index++) {
            int difference =
                primeNumbers.get(index) - primeNumbers.get(index - 1);
            if (difference < minDifference) {
                minDifference = difference;
                closestPair = new int[] {
                    primeNumbers.get(index - 1),
                    primeNumbers.get(index),
                };
            }
        }
        return closestPair;
    }
    private boolean isPrime(int number) {
        if (number == 1) {
            return false;
        }
        for (int divisor = 2; divisor <= (int) Math.sqrt(number); divisor++) {
            if (number % divisor == 0) {
                return false;
            }
        }
        return true;
    }
}
