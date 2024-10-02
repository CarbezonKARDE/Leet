use std::collections::HashMap;
impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut sorted_arr = arr.clone();
        sorted_arr.sort();
        sorted_arr.dedup();
        let mut rank_map = HashMap::new();
        for (rank, &val) in sorted_arr.iter().enumerate() {
            rank_map.insert(val, (rank + 1) as i32);
        }
        arr.into_iter().map(|x| rank_map[&x]).collect()
    }
}
