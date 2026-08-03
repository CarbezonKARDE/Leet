impl Solution {
    pub fn stone_game_iii(s: Vec<i32>) -> String {
        ["Bob", "Tie", "Alice"][(s.into_iter().rfold([0; 5], |[a, b, c, u, v], x| {
            [(x - a).max(x + u - b).max(x + u + v - c), a, b, x, u]
        })[0].signum() + 1) as usize].into()
    }
}
