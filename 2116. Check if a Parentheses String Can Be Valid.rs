impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }
        let START = '(' as u8;
        let mut open: Vec<usize> = Vec::new();
        let mut unlocked: Vec<usize> = Vec::new();
        let res = s
          .as_bytes()
          .iter()
          .zip(locked.as_bytes().iter())
          .map(|(&c, &l)| (if c == START {true} else {false}, if l == '1' as u8 {true} else {false}))
          .enumerate()
          .map(|(i, (is_start, is_locked))| {
            if !is_locked {
                unlocked.push(i);
                true
            } else if is_start {
                open.push(i);
                true
            } else {
                match (open.last(), unlocked.last()) {
                    (Some(&o), _) => {
                        open.pop();
                        true
                    },
                    (None, Some(&u)) => {
                        unlocked.pop();
                        true
                    },
                    _ => false,
                }
            }
          })
          .all(|ok| ok);
        if !res {
            return res;
        }
        std::iter::successors(open.last().copied().zip(unlocked.last().copied()), |(o, u)| {
            if o < u {
                open.pop();
                unlocked.pop();
                open.last().copied().zip(unlocked.last().copied())
            } else {
                None
            }
        })
        .last();
        open.is_empty()
    }
}
