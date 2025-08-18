const epsl: f32 = 10.0e-5;
impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        let mut vec = Vec::new();
        for card in cards {
            vec.push(card as f32);
        }
        return Self::check(&vec);
    }
    fn check(vec: &Vec<f32>) -> bool {
        if vec.len() == 1 {
            return (vec[0] - 24.0).abs() <= epsl;
        }
        for  i in 0..vec.len() {
            for j in 0..vec.len() {
                if i == j {continue;}
                let mut res: Vec<f32> = vec![];
                for k in 0..vec.len() {
                    if k != i && k != j {
                        res.push(vec[k]);
                    }
                }
                for op in vec!['+', '-', '*', '/'] {
                    if (op == '+' || op == '*') && i > j {
                        continue;
                    }
                    if op == '/' && vec[j] == 0.0 {
                        continue;
                    }
                    match op {
                        '+' => res.push(vec[i] + vec[j]),
                        '-' => res.push(vec[i] - vec[j]),
                        '*' => res.push(vec[i] * vec[j]),
                        '/' => res.push(vec[i] / vec[j]),
                        _ => todo!()
                    }
                    if Self::check(&res) {
                        return true;
                    }
                    res.pop();
                }
            }
        }
        false
    }
}
