use itertools::Itertools;
impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut sorted_vowels = s
            .chars()
            .filter_map(|c| if "AEIOUaeiou".contains(c) { Some(c) } else { None })
            .sorted_by_key(|&c| c as usize);
        s.chars().map(|x| {
            if "AEIOUaeiou".contains(x) { sorted_vowels.next().unwrap() }
            else { x } 
        }).collect()
    }
}
