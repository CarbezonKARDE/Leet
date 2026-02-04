impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        let nums: Vec<i64> = nums.into_iter().map(i64::from).collect();
        let n = nums.len();
        const NEG_INF: i64 = i64::MIN / 2;
        let mut s0 = NEG_INF;
        let mut s1 = NEG_INF;
        let mut s2 = NEG_INF;
        let mut ans = NEG_INF;
        (1..n).for_each(|i| {
            let (new_s0, new_s1, new_s2) = match nums[i].cmp(&nums[i - 1]) {
                std::cmp::Ordering::Greater => {
                    (
                        nums[i - 1].max(s0) + nums[i],
                        NEG_INF,
                        s1.max(s2) + nums[i],
                    )
                }
                std::cmp::Ordering::Less => {
                    (
                        NEG_INF,
                        s0.max(s1) + nums[i],
                        NEG_INF,
                    )
                }
                std::cmp::Ordering::Equal => {
                    (NEG_INF, NEG_INF, NEG_INF)
                }
            };
            s0 = new_s0;
            s1 = new_s1;
            s2 = new_s2;
            ans = ans.max(s2);
        });

        ans
    }
}
