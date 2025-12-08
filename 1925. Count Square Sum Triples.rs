impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut ans = 0;
        let mut u = 3;
        while u * u < n * 2 {
            let mut v = 1;
            while v < u && u * u + v * v <= n * 2 {
                if gcd(u, v) == 1 {
                    ans += n * 2 / (u * u + v * v);
                }
                v += 2;
            }
            u += 2;
        }
        ans * 2
    }
}
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a != 0 {
        (a, b) = (b % a, a);
    }
    b
}
