impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut f = [0;501];
        for i in arr{
            f[i as usize]+=1;
        }
        for i in (1..=500).rev(){
            if f[i as usize]==i as usize{
                return i as i32;
            }
        }
        return -1;
    }
}
