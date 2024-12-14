impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        fn lg2(x: usize) -> usize {
            if x == 0 {
                0
            } else {
                (x as f64).log2().floor() as usize
            }
        }
        fn build_min_table(nums: &Vec<i32>) -> Vec<Vec<i32>> {
            let n = nums.len();
            let max_k = lg2(n) + 1;
            let mut min_table = vec![vec![0; n]; max_k];
            min_table[0].clone_from_slice(nums);
            for k in 1..max_k {
                for i in 0..(n - (1 << k) + 1) {
                    min_table[k][i] = min_table[k - 1][i].min(min_table[k - 1][i + (1 << (k - 1))]);
                }
            }
            min_table
        }
        fn build_max_table(nums: &Vec<i32>) -> Vec<Vec<i32>> {
            let n = nums.len();
            let max_k = lg2(n) + 1;
            let mut max_table = vec![vec![0; n]; max_k];
            max_table[0].clone_from_slice(nums);
            for k in 1..max_k {
                for i in 0..(n - (1 << k) + 1) {
                    max_table[k][i] = max_table[k - 1][i].max(max_table[k - 1][i + (1 << (k - 1))]);
                }
            }
            max_table
        }
        fn query_min(table: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
            let k = lg2(y - x + 1);
            table[k][x].min(table[k][y - (1 << k) + 1])
        }
        fn query_max(table: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
            let k = lg2(y - x + 1);
            table[k][x].max(table[k][y - (1 << k) + 1])
        }
        let min_table = build_min_table(&nums);
        let max_table = build_max_table(&nums);
        let mut result: i64 = 0;
        for i in 0..n {
            let mut low = i;
            let mut high = n - 1;
            while low < high {
                let mid = (low + high + 1) / 2;
                let min_val = query_min(&min_table, i, mid);
                let max_val = query_max(&max_table, i, mid);
                if max_val - min_val > 2 {
                    high = mid - 1;
                } else {
                    low = mid;
                }
            }
            result += (high - i + 1) as i64;
        }
        result
    }
}
