impl Solution {
    pub fn xor_queries(mut arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        for i in 1..arr.len() {
            arr[i] ^= arr[i - 1];
        }
        queries.iter().map(|q| {
            let (start, end) = (q[0] as usize, q[1] as usize);
            if start > 0 {
                arr[end] ^ arr[start - 1]
            } else {
                arr[end]
            }
        }).collect()
    }
}
