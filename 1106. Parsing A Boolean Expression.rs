impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut stack = Vec::new();
        for curr_char in expression.chars() {
            if curr_char == ',' || curr_char == '(' {
                continue;
            }
            if "tf!&|".contains(curr_char) {
                stack.push(curr_char);
            } else if curr_char == ')' {
                let mut has_true = false;
                let mut has_false = false;
                while !matches!(stack.last(), Some('!') | Some('&') | Some('|')) {
                    let top_value = stack.pop().unwrap();
                    if top_value == 't' {
                        has_true = true;
                    }
                    if top_value == 'f' {
                        has_false = true;
                    }
                }
                let op = stack.pop().unwrap();
                match op {
                    '!' => stack.push(if has_true { 'f' } else { 't' }),
                    '&' => stack.push(if has_false { 'f' } else { 't' }),
                    '|' => stack.push(if has_true { 't' } else { 'f' }),
                    _ => {}
                }
            }
        }
        stack.pop().unwrap() == 't'
    }
}
