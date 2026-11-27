impl Solution {
    pub fn can_alice_win(mut n: i32) -> bool {
        match n % 3 {
            0 => {
                let mut answer = false;
                let mut diff = 10;
                while n > 0 && diff > 0 {
                    n -= diff;
                    diff -= 1;
                    answer = !answer;
                }

                if n == 0 { answer } else { !answer }
            }
            1 => match n {
                ..10 => false,
                10..19 => true,
                19..27 => false,
                27..34 => true,
                34..40 => false,
                40..45 => true,
                45..49 => false,
                49..52 => true,
                52..54 => false,
                54.. => true,
            },
            _ => {
                let bla = (433 - 8 * n).isqrt() - 1;
                bla % 4 < 2
            }
        }
    }
}
