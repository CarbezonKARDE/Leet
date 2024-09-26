#[derive(Default)]
struct MyCalendar {
    events: std::collections::BTreeSet<(i32, i32)>,
}
impl MyCalendar {
    fn new() -> Self {
        Default::default()
    }
    fn book(&mut self, start: i32, end: i32) -> bool {
        let l = self.events.range((start, 0)..).next().map_or(true, |&(s, _)| s >= end);
        let r = self.events.range(..(start, 0)).rev().next().map_or(true, |&(_, e)| e <= start);
        l && r && self.events.insert((start, end))
    }
}
