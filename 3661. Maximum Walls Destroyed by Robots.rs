impl Solution {
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, walls: Vec<i32>) -> i32 {
        let n = robots.len();
        let m = walls.len();
        let mut ss = Vec::with_capacity(n + m + 2);
        for (r, d) in robots.into_iter().zip(distance) {
            ss.push((r, d));
        }
        ss.push((0, 0));
        ss.push((i32::MAX, 0));
        for w in walls {
            ss.push((w, i32::MAX));
        }
        ss.sort_unstable();
        let mut res0 = 0;
        let mut last = 0;
        ss.retain(|&(x, d)| {
            if d == i32::MAX && last == x {
                res0 += 1;
                return false;
            }
            last = x;
            true
        });
        let mut i = 0;
        let mut dp_l = 0;
        let mut dp_r = 0;
        loop {
            let (lx, ld) = ss[i];
            if lx == i32::MAX { break; }
            let lxx = lx + ld;
            let mut lc = 0;
            let mut rx = 0;
            let mut rd = 0;
            let mut j = i + 1;
            loop {
                (rx, rd) = ss[j];
                if rd < i32::MAX { break; }
                if rx <= lxx {
                    lc += 1;
                }
                j += 1;
            }
            let mut c = (j - i - 1) as i32;
            let mut rc = 0;
            let rxx = rx - rd;
            for k in (i + 1..j).rev() {
                if ss[k].0 >= rxx {
                    rc += 1;
                } else {
                    break;
                }
            }
            let dp_l_1 = (dp_l + rc).max(dp_r + c.min(lc + rc));
            let dp_r_1 = dp_l.max(dp_r + lc);
            dp_l = dp_l_1;
            dp_r = dp_r_1;
            i = j;
        }
        dp_l.max(dp_r) + res0
    }
}
