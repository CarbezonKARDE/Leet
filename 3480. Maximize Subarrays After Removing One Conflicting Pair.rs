impl Solution {
    pub fn max_subarrays(n: i32, mut conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        for pair in conflicting_pairs.iter_mut() {
            if pair[0] > pair[1] {
                pair.swap(0, 1);
            }
        }
        conflicting_pairs.sort_unstable_by_key(|p| -p[0]);
        let mut conflict_pairs_iter = conflicting_pairs.into_iter().peekable();
        let mut right_boundary = n + 1;
        let mut largest_correction = 0;
        let mut valid_subarrays: i64 = 0;
        let mut cur_penalty: i64 = 0;
        let mut max_penalty: i64 = 0;
        for b in (1..=n).rev() {
            while let Some(p) = conflict_pairs_iter.next_if(|p| p[0] >= b) {
                if right_boundary > p[1] {
                    largest_correction = right_boundary - p[1];
                    right_boundary = p[1];
                    cur_penalty = 0;
                } else {
                    largest_correction = largest_correction.min(p[1] - right_boundary);
                }
            }
            valid_subarrays += (right_boundary - b) as i64;
            cur_penalty += largest_correction as i64;
            max_penalty = max_penalty.max(cur_penalty);
        }
        valid_subarrays + max_penalty
    }
}
