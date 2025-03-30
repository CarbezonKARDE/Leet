use std::collections::HashMap;
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut hash:HashMap<char,usize> = HashMap::new();
        let mut chars = vec![];
        for (idx,ch) in s.chars().enumerate() {
            chars.push(ch);
            if hash.contains_key(&ch) {
                hash.insert(ch,hash[&ch].max(idx));
            } else {
                hash.insert(ch,idx);
            }
        }
        let (mut start,mut cur,mut end,mut result,mut max) = (0,0,chars.len(),vec![],hash[&chars[0]]);
        while start <= end {
            while cur < max {
                end = hash[&chars[cur]];
                max = max.max(end);
                cur += 1;
            }
            result.push((max-start+1) as i32);
            start = max+1;
            if start>=chars.len() {break}
            (cur,end,max) = (start,hash[&chars[start]],hash[&chars[start]]);
        }
        if end>=start { result.push((start-end+1) as i32) }
        result
    }
}
