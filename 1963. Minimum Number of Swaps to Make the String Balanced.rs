impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut stack_size = 0;        
        for ch in s.chars() {
            if ch == '[' {
                stack_size += 1;
            } else if ch == ']' {
                if stack_size > 0 {
                    stack_size -= 1;
                }
            }
        }        
        (stack_size + 1) / 2
    }
}
