impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut scores = vec![0];
        for p in s.chars() {
            if p == '(' {
                scores.push(0);
            } else {
                let s1 = scores.pop().unwrap();
                let s0 = scores.pop().unwrap();

                scores.push((2 * s1).max(1) + s0);
            }
        }
        scores.pop().unwrap()
    }
}
