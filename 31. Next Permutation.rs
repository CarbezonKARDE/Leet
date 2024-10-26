impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let l = nums.len();
        let mut k = 0;
        for i in (1..l).rev() {
            if nums[i - 1] < nums[i] {
                k = i;
                break;
            }
        }
        if k == 0 {
            nums.reverse();
        } else {
            let t = nums[k - 1];
            for i in k..l {
                if t >= nums[i] {
                    nums.swap(k - 1, i - 1);
                    break;
                }
                if i == l - 1 {
                    nums.swap(k - 1, i);
                }
            }
            nums[k..].reverse();
        }
    }
}
