impl Solution {
    pub fn maximize_square_area(total_height: i32, total_width: i32, mut horizontal: Vec<i32>, mut vertical: Vec<i32>) -> i32 {
        const MOD:i64 = 1_000_000_007;
        fn prep(mut cuts:Vec<i32>, limit:i32)->Vec<i32>{
            cuts.sort();
            let mut out=vec![1];
            out.extend(cuts);
            out.push(limit);
            out
        }
        let h = prep(horizontal, total_height);
        let v = prep(vertical, total_width);
        let mut set = std::collections::HashSet::new();
        for i in 0..h.len() {
            for j in i+1..h.len() {
                set.insert(h[j]-h[i]);
            }
        }
        let mut best = 0;
        for i in 0..v.len() {
            for j in i+1..v.len() {
                let d = v[j]-v[i];
                if d>best && set.contains(&d) {
                    best = d;
                }
            }
        }
        if best==0 { return -1; }
        return ((best as i64 * best as i64)%MOD) as i32;
    }
}
