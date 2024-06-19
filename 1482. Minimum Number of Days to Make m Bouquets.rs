impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        if (bloom_day.len() as i64) < (m as i64) * (k as i64) {
            return -1;
        }
        let (mut l, mut r) = (*bloom_day.iter().min().unwrap(), *bloom_day.iter().max().unwrap());
        while l < r {
            let mid = (l + r) / 2;
            if Self::get_bouquet_count(&bloom_day, k, mid) >= m {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l
    }
    fn get_bouquet_count(bloom_day: &Vec<i32>, k: i32, waiting_days: i32) -> i32 {
        let mut bouquet_count = 0;
        let mut required_flowers = k;
        for &day in bloom_day {
            if day > waiting_days {
                required_flowers = k;
            } else {
                required_flowers -= 1;
                if required_flowers == 0 {
                    bouquet_count += 1;
                    required_flowers = k;
                }
            }
        }
        bouquet_count
    }
}
