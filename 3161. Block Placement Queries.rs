impl Solution {
    pub fn get_results(q: Vec<Vec<i32>>) -> Vec<bool> {
        fn a(b: &mut [usize], mut i: usize, x: usize) {
            while i < 50002 {
                if b[i] < x {
                    b[i] = x;
                }
                i += i & !i + 1;
            }
        }
        fn f(p: &mut [usize], mut x: usize) -> usize {
            while p[x] < x {
                let y = p[x];
                p[x] = p[y];
                x = y;
            }
            x
        }
        let [mut p, mut n, mut b] = [[0; 50002]; 3];
        for v in &q {
            p[v[1] as usize] |= 2 - v[0] as usize
        }
        p[50001] = 1;
        let mut z = 0;
        for x in 1..50002 {
            if p[x] > 0 {
                a(&mut b, x, x - z);
                n[z] = x;
                z = x;
            }
            p[x] = z;
        }
        let mut r = vec![];
        for v in q.iter().rev() {
            let x = v[1] as _;
            if v[0] < 2 {
                let l = f(&mut p, x - 1);
                let y = n[x];
                a(&mut b, y, y - l);
                p[x] = l;
                n[l] = y
            } else {
                let mut i = f(&mut p, x);
                let mut m = x - i;
                while i > 0 {
                    m = m.max(b[i]);
                    i &= i - 1
                }
                r.push(m >= v[2] as _)
            }
        }
        r.reverse();
        r
    }
}
