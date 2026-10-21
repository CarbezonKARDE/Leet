impl Solution {
    pub fn min_operations(s: String) -> i32 {
        if s.as_bytes().is_sorted() {
            return 0;
        }
        if s.len() == 2 {
            return -1;
        }
        let (l, mut mn, mut mx, mut ni, mut xi) = (s.len() as u32 - 1, 255, 0, 0, 0);
        for (&n, i) in s.as_bytes().iter().zip(0u32..) {
            if n >= mx {
                mx = n;
                xi = i;
            }
            if n < mn {
                mn = n;
                ni = i;
            }
        }
        2 + i32::from(ni == l && xi == 0) - i32::from(ni == 0 || xi == l)
    }
}
