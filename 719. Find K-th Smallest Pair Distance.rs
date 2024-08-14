impl Solution {
    pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
        let mut distances:Vec<i32>=Vec::new();
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                distances.push((nums[i]-nums[j]).abs());
            }
        }
        distances.sort_unstable();
        distances[(k-1) as usize]
    }
}
