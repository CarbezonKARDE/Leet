use std::collections::{HashMap, BTreeSet};
use std::cmp::max;
impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        const K_MAX_NUM: usize = 1000;
        let mut count = vec![0; K_MAX_NUM + 1];
        let mut mod_to_subset: HashMap<i32, BTreeSet<i32>> = HashMap::new();

        for &num in &nums {
            count[num as usize] += 1;
            mod_to_subset.entry(num % k).or_insert_with(BTreeSet::new).insert(num);
        }
        let mut prev_num = -k;
        let mut skip = 0;
        let mut pick = 0;
        for (_, subset) in mod_to_subset {
            for &num in &subset {
                let non_empty_count = (1 << count[num as usize]) - 1;
                let cache_skip = skip;
                skip += pick;
                pick = non_empty_count * (1 + cache_skip + if num - prev_num == k { 0 } else { pick });
                prev_num = num;
            }
        }
        skip + pick
    }
}
