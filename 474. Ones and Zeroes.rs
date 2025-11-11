impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut tmp = Vec::with_capacity(strs.len());
        let mut active = vec![(0, 0)];
        let mut best = vec![vec![0; 1 + m as usize]; 1 + n as usize];
        let mut ans = 0;
        for s in strs {
            let (oc, zc) = s.chars().fold((0, 0), |(oc, zc), c| match c {
                '0' => (oc, zc + 1),
                _ => (oc + 1, zc),
            });
            for &(of, zf) in active.iter() {
                if of + oc > n || zf + zc > m {
                    continue;
                }
                tmp.push((of + oc, zc + zf, best[of][zf] + 1));
            }
            for &(of, zf, c) in tmp.iter() {
                if best[of][zf] == 0 {
                    active.push((of, zf));
                }
                best[of][zf] = best[of][zf].max(c);
                ans = ans.max(best[of][zf]);
            }
            tmp.clear()
        }
        ans
    }
}
