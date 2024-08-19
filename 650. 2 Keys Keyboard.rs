impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        fn find_min_steps(current_length: i32, clipboard_length: i32, target_length: i32) -> i32 {
            if current_length == target_length {
                return 0;
            }
            if current_length > target_length {
                return i32::MAX / 2;
            }
            let copy_and_paste = 2 + find_min_steps(current_length * 2, current_length, target_length);
            let paste_only = 1 + find_min_steps(current_length + clipboard_length, clipboard_length, target_length);
            copy_and_paste.min(paste_only)
        }
        1 + find_min_steps(1, 1, n)
    }
}
