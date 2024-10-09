impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut open_c = 0;
        let mut close_c = 0;
        for c in s.chars() {
            if c == '(' {
                open_c += 1;
            } else if c == ')' && open_c > 0 {
                open_c -= 1;
            } else {
                close_c += 1;
            }
        }
        open_c + close_c
    }
}
