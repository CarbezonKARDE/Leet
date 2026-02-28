impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        ANS[n as usize]
    }
}
const ANS: [i32; 100_000 + 1] = {
    let mut ans = [0; 100_000 + 1];
    let mut a = 0_usize;
    let mut n = 1_usize;
    loop {
        let nlen = usize::BITS - n.leading_zeros();
        a = (a << nlen) | n;
        a %= 1_000_000_007;
        ans[n] = a as _;
        n += 1;
        if n == ans.len() {
            break ans;
        }
    }
};
