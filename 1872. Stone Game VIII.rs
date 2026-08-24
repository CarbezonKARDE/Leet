impl Solution {
    pub fn stone_game_viii(mut s: Vec<i32>) -> i32 {
        for i in 1..s.len() { s[i] += s[i - 1] }
        let last = s.pop().unwrap();
        s[1..].iter().rfold(last, |ans, &x| ans.max(x - ans))
    }
}
