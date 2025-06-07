impl Solution {
    pub fn clear_stars(mut s: String) -> String {
        let b = unsafe { s.as_mut_vec() };
        let mut d = vec![Vec::new(); 26];
        for i in 0..b.len() {
            if b[i] != b'*' {
                d[(b[i] - b'a') as usize].push(i as u32);
            } else {
                b[i] = 0;
                for stk in &mut d {
                    if let Some(pos) = stk.pop() {
                        b[pos as usize] = 0;
                        break;
                    }
                }
            }
        }
        let mut write = 0;
        for read in 0..b.len() {
            if b[read] != 0 {
                b[write] = b[read];
                write += 1;
            }
        }
        unsafe { b.set_len(write); }
        s
    }
}
