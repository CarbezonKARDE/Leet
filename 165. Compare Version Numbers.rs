use std::str::FromStr;
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut iter1 = version1.split('.').map(|s| i32::from_str(s).unwrap_or(0));
        let mut iter2 = version2.split('.').map(|s| i32::from_str(s).unwrap_or(0));
        loop {
            match (iter1.next(), iter2.next()) {
                (Some(v1), Some(v2)) => {
                    if v1 < v2 {
                        return -1;
                    } else if v1 > v2 {
                        return 1;
                    }
                }
                (None, Some(v2)) => {
                    if v2 > 0 {
                        return -1;
                    }
                }
                (Some(v1), None) => {
                    if v1 > 0 {
                        return 1;
                    }
                }
                (None, None) => break,
            }
        }
        0
    }
}
