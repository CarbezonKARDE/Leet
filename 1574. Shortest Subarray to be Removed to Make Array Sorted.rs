impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len();        
        let mut right = n - 1;
        while right > 0 && arr[right - 1] <= arr[right] {
            right -= 1;
        }        
        if right == 0 {
            return 0;
        }        
        let mut min_length = right;        
        for left in 0..n {
            if left > 0 && arr[left - 1] > arr[left] {
                break;
            }            
            while right < n && arr[left] > arr[right] {
                right += 1;
            }            
            let current_length = right - left - 1;
            min_length = min_length.min(current_length);
        }
        min_length as i32
    }
}
