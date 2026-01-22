impl Solution {
    pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        while nums.len() > 1 {
            if nums.windows(2).all(|w| w[0] <= w[1]) { break };
            let sums = nums.windows(2).map(|w| w[0] + w[1]).enumerate();
            let (minn, minx) = sums.clone().min_by_key(|(_, x)| *x).unwrap();
            nums = nums.iter().enumerate().filter(|&(i, x)| i != minn).map(|(i, &x)| {
                if i == minn + 1 { minx } else { x }
            }).collect::<Vec<i32>>();
            ans += 1;
        }
        ans
    }
}
