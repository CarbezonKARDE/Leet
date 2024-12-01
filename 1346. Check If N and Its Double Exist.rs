impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut zero_qty = 0i32;
        for index in 0..arr.len() {
            if arr[index] == 0 {
                zero_qty += 1;
                if zero_qty >= 2 {
                    return true;
                } 
                continue;
            }
            if arr[index] % 2 != 0 {
                continue;
            }
            let i: i32 = arr[index] / 2;
            if arr.contains(&i) {
                return true;
            }
        }
        false
    }
}
