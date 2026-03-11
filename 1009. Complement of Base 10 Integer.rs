impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        let mut m = n;
        let mut manipulator = 1;
        if n==0 {
            return 1 as i32;
        }
        while m != 0 {
            manipulator = manipulator<<1;
            m = m>>1;
        }
        let res = (manipulator-1) - n;
        res
    }
}
