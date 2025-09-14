use std::collections::HashMap;
type Query = u64;
type WordMap = HashMap<Query, usize>;
impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, mut queries: Vec<String>) -> Vec<String> {
        let (map1, map2, map3) = Self::build_wordmaps(&wordlist);
        for i in 0..queries.len() {
            let query = Self::to_query(&queries[i]);
            let word_idx = Self::find_word(query, &map1, &map2, &map3);
            queries[i] = if word_idx < usize::MAX {
                wordlist[word_idx].clone()
            } else {
                String::new()
            };
        }
        queries
    }
    fn to_query(word: &str) -> u64 {
        let mut res = 0;
        for byte in word.bytes() {
            res = (res << 6) | (byte as u64 & 0x3f);
        }
        res
    }
    #[inline]
    fn build_wordmaps(wordlist: &[String]) -> (WordMap, WordMap, WordMap) {
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        let mut map3 = HashMap::new();
        for (idx, word) in wordlist.into_iter().enumerate().rev() {
            let mut query = Self::to_query(&word);
            map1.insert(query, idx);
            query = Self::remove_capitalization(query);
            map2.insert(query, idx);
            query = Self::remove_vowel_errors(query);
            map3.insert(query, idx);
        }
        (map1, map2, map3)
    }
    #[inline]
    fn remove_capitalization(query: Query) -> Query {
        query | 0x20820820820
    }
    #[inline]
    fn is_vowel(byte: u8) -> bool {
        const MASK: u32 = 0x208222;
        ((MASK >> byte) & 1) != 0
    }
    #[inline]
    fn remove_vowel_errors(mut query: Query) -> Query {
        let (mut res, mut shift) = (0, 0);
        while query > 0 {
            let byte = query & 0x3f;
            if Self::is_vowel(byte as u8) {
                res |= (b'a' as u64 & 0x3f) << shift;
            } else {
                res |= byte << shift;
            }
            query >>= 6;
            shift += 6;
        }
        res
    }
    #[inline]
    fn find_word(mut query: u64, map1: &WordMap, map2: &WordMap, map3: &WordMap) -> usize {
        if let Some(idx) = map1.get(&query) {
            return *idx;
        }
        query = Self::remove_capitalization(query);
        if let Some(idx) = map2.get(&query) {
            return *idx;
        }
        query = Self::remove_vowel_errors(query);
        if let Some(idx) = map3.get(&query) {
            return *idx;
        }
        usize::MAX
    }
}
