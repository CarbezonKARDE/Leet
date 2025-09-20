use std::collections::{VecDeque, HashSet, HashMap};
struct Router {
    data: VecDeque<(i32, i32, i32)>,
    uniq: HashSet<(i32, i32, i32)>,
    bydest: HashMap<i32, VecDeque<i32>>,
    n: usize,
}
impl Router {
    fn new(memoryLimit: i32) -> Self {
        Self { 
            data: VecDeque::with_capacity(memoryLimit as usize),
            uniq: HashSet::new(),
            n: memoryLimit as usize,
            bydest: HashMap::new(),
        }
    }
    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let res = if !self.uniq.insert((source, destination, timestamp)) {
            false
        } else {
            if self.data.len() == self.n {
                self.pop_packet();
            }
            self.data.push_back((source, destination, timestamp));
            self.bydest.entry(destination).or_insert_with(|| VecDeque::new()).push_back(timestamp);
            true
        };
        res
    }
    fn pop_packet(&mut self) -> Option<(i32, i32, i32)> {
        if let Some((source, destination, timestamp)) = self.data.pop_front() {
            self.uniq.remove(&(source, destination, timestamp));
            self.bydest.entry(destination).and_modify(|x| { x.pop_front(); });
            Some((source, destination, timestamp))
        } else {
            None
        }
    }
    fn forward_packet(&mut self) -> Vec<i32> {
        let res = if let Some((source, destination, timestamp)) = self.pop_packet() {
            vec![source, destination, timestamp]
        } else {
            Vec::new()
        };
        res
    }
    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        if let Some(d) = self.bydest.get(&destination) {
            let pp1 = d.partition_point(|&x| x < start_time);
            let pp2 = d.partition_point(|&x| x <= end_time);
            (pp2 - pp1) as i32
        } else {
            0
        }
    }
}
