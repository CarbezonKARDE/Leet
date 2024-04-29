impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let n1 = nums1.len();
        let n2 = nums2.len();
        if n1 > n2 {
            return Solution::find_median_sorted_arrays(nums2, nums1);
        }
        let mut l = 0;
        let mut r = n1;
        while l <= r {
            let partition1 = (l + r) / 2;
            let partition2 = (n1 + n2 + 1) / 2 - partition1;
            let max_left1 = if partition1 == 0 { i32::MIN } else { nums1[partition1 - 1] };
            let max_left2 = if partition2 == 0 { i32::MIN } else { nums2[partition2 - 1] };
            let min_right1 = if partition1 == n1 { i32::MAX } else { nums1[partition1] };
            let min_right2 = if partition2 == n2 { i32::MAX } else { nums2[partition2] };
            if max_left1 <= min_right2 && max_left2 <= min_right1 {
                return if (n1 + n2) % 2 == 0 {
                    ((max_left1.max(max_left2) + min_right1.min(min_right2)) as f64) * 0.5
                } else {
                    max_left1.max(max_left2) as f64
                };
            } else if max_left1 > min_right2 {
                r = partition1 - 1;
            } else {
                l = partition1 + 1;
            }
        }
        panic!();
    }
}
