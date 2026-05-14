impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
        let n = *nums.iter().max().unwrap();
        let mut freq_map = vec![0; (n + 1) as usize];
        for num in nums {
            freq_map[num as usize] += 1;
        }
        if freq_map[n as usize] != 2 {
            return false;
        }
        for i in 1..n {
            if freq_map[i as usize] != 1 {
                return false;
            }
        }
        return true;
    }
}
