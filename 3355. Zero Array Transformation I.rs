impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let mut delta_array = vec![0; nums.len() + 1];
        for query in queries {
            let left = query[0] as usize;
            let right = query[1] as usize;
            delta_array[left] += 1;
            delta_array[right + 1] -= 1;
        }
        let mut operation_counts = Vec::with_capacity(delta_array.len());
        let mut current_operations = 0;
        for delta in delta_array {
            current_operations += delta;
            operation_counts.push(current_operations);
        }
        for i in 0..nums.len() {
            if operation_counts[i] < nums[i] {
                return false;
            }
        }
        true
    }
}
