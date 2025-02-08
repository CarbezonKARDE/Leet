use std::collections::{HashMap,BTreeSet};
struct NumberContainers {
    hash: HashMap<i32,i32>,
    numbers: HashMap<i32,BTreeSet<i32>>
}
impl NumberContainers {
    fn new() -> Self {
        NumberContainers{hash:HashMap::new(),numbers:HashMap::new()}
    }
    fn change(&mut self, index: i32, number: i32) {
        if self.hash.contains_key(&index) {
            if self.numbers.contains_key(&self.hash[&index]) {
                self.numbers.get_mut(&self.hash[&index]).unwrap().remove(&index);
            }
        }
        *self.hash.entry(index).or_insert(number) = number;
        self.numbers.entry(number).or_insert(BTreeSet::new()).insert(index);
    }
    fn find(&self, number: i32) -> i32 {
        if self.numbers.contains_key(&number) {
            if let Some(idx) = self.numbers[&number].first() {
                return *idx;
            }
        }
        -1
    }
}
