impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let n = s.len();
        let a = a as i32;
        let b = b as usize;
        let mut best_add = vec![0usize; 10];
        for d in 0..10 {
            let mut min_val = d as i32;
            let mut min_step = 0usize;
            for step in 1..10 {
                let new_val = ((d as i32) + a * (step as i32)) % 10;
                if new_val < min_val {
                    min_val = new_val;
                    min_step = step as usize;
                }
            }
            best_add[d] = min_step;
        }
        let mut visited = vec![false; n];
        let mut idx = 0usize;
        while !visited[idx] {
            visited[idx] = true;
            idx = (idx + b) % n;
        }
        let mut answer = s.clone();
        for start in 0..n {
            if !visited[start] { continue; }
            let rotated = format!("{}{}", &s[start..], &s[..start]);
            let (even_add, odd_add) = if n == 1 {
                (best_add[(rotated.as_bytes()[0] - b'0') as usize], 0usize)
            } else {
                let even = if b % 2 == 1 {
                    best_add[(rotated.as_bytes()[0] - b'0') as usize]
                } else { 0usize };
                let odd = best_add[(rotated.as_bytes()[1] - b'0') as usize];
                (even, odd)
            };
            let mut chars: Vec<u8> = rotated.into_bytes();
            for j in 0..n {
                let d = (chars[j] - b'0') as i32;
                let times = if j % 2 == 0 { even_add } else { odd_add };
                let nd = ((d + (times as i32) * a) % 10 + 10) % 10;
                chars[j] = (b'0' + nd as u8) as u8;
            }
            let candidate = String::from_utf8(chars).unwrap();
            if candidate < answer {
                answer = candidate;
            }
        }
        answer
    }
}
