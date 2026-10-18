impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        let mod_val: i64 = 1_000_000_007;
        let n = nums.len();
        let mut combinations = vec![vec![0; n + 1]; n + 1];
        for i in 0..=n {
            combinations[i][0] = 1;
            for j in 1..=i {
                combinations[i][j] = (combinations[i - 1][j - 1] + combinations[i - 1][j]) % mod_val;
            }
        }
        fn get_ways(arr: &[i32], combinations: &Vec<Vec<i64>>, mod_val: i64) -> i64 {
            if arr.len() <= 2 { return 1; }
            let root = arr[0];
            let mut left = Vec::new();
            let mut right = Vec::new();
            for i in 1..arr.len() {
                if arr[i] < root { left.push(arr[i]); }
                else { right.push(arr[i]); }
            }
            let left_ways = get_ways(&left, combinations, mod_val);
            let right_ways = get_ways(&right, combinations, mod_val);
            let ways = combinations[left.len() + right.len()][left.len()];
            (((ways * left_ways) % mod_val) * right_ways) % mod_val
        }
        ((get_ways(&nums, &combinations, mod_val) - 1 + mod_val) % mod_val) as i32
    }
}
