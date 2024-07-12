impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        if x > y {
            return Self::gain(&s, "ab", x, "ba", y);
        } else {
            return Self::gain(&s, "ba", y, "ab", x);
        }
    }
    fn gain(s: &str, sub1: &str, point1: i32, sub2: &str, point2: i32) -> i32 {
        let mut points = 0;
        let mut stack1 = Vec::new();
        let mut stack2 = Vec::new();
        for c in s.chars() {
            if !stack1.is_empty() && stack1.last().unwrap() == &sub1.chars().next().unwrap() && c == sub1.chars().nth(1).unwrap() {
                stack1.pop();
                points += point1;
            } else {
                stack1.push(c);
            }
        }
        for c in stack1 {
            if !stack2.is_empty() && stack2.last().unwrap() == &sub2.chars().next().unwrap() && c == sub2.chars().nth(1).unwrap() {
                stack2.pop();
                points += point2;
            } else {
                stack2.push(c);
            }
        }
        points
    }
}
