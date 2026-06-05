impl Solution {
    pub fn total_waviness(a: i64, b: i64) -> i64 {
        fn f(x: i64) -> i64 {
            let mut d = [(0, 0); 66];
            d[65].0 = 1;
            for p in x.to_string().bytes() {
                let p = p as usize - 48;
                let mut e = [(0, 0); 66];
                for i in 0..66 {
                    let (c, s) = d[i];
                    let (m, q, g) = (i / 6, i / 2 % 3, i % 2 > 0);
                    for r in 0..=if g { p } else { 9 } {
                        let nm = if m > 9 && r < 1 { 10 } else { r };
                        let nq = if m > 9 || m == r {
                            2
                        } else {
                            (m > r) as usize
                        };
                        let n = nm * 6
                            + nq * 2
                            + (g && r == p) as usize;
                        e[n].0 += c;
                        e[n].1 += s
                            + (q < 1 && r < m || q == 1 && r > m) as i64 * c;
                    }
                }
                d = e;
            }
            d.iter().map(|x| x.1).sum()
        }
        f(b) - f(a - 1)
    }
}
