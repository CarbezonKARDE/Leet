impl Solution {
    pub fn process_str(s: String) -> String {
        let mut stack: Vec<char> = vec![];
        for ch in s.chars() {
            match ch {
                '*' => if stack.len() > 0 { stack.pop(); },
                '#' => stack.extend_from_within(..),
                '%' => stack.reverse(),
                 _  => stack.push(ch)
            }
        }
        String::from_iter(stack)
    }
}
