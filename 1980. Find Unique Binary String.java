import java.util.HashSet;
import java.util.Set;

class Solution {
    public String findDifferentBinaryString(String[] nums) {
        final int bitSize = nums[0].length();
        Set<String> numsSet = new HashSet<>();

        for (String num : nums)
            numsSet.add(num);

        StringBuilder sb = new StringBuilder();

        for (int i = 0; i < (1 << bitSize); i++) {
            String binary = Integer.toBinaryString(i);
            while (binary.length() < bitSize) {
                binary = "0" + binary;
            }
            if (!numsSet.contains(binary))
                return binary;
        }

        return "";
    }
}
