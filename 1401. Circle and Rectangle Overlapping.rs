impl Solution {
    pub fn check_overlap(r: i32, x: i32, y: i32, a: i32, b: i32, c: i32, d: i32) -> bool {
        (x-x.clamp(a,c)).pow(2)+(y-y.clamp(b,d)).pow(2)<=r*r
    }
}
