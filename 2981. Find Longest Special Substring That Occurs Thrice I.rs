impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        use std::collections::HashMap;
        let freq = s
          .chars()
          .scan(None, |last, c| {
            match *last {
                None => {
                    *last = Some((c, 1));
                },
                Some((c_p, k)) => {
                    if c == c_p {
                        *last = Some((c, k + 1));
                    } else {
                        *last = Some((c, 1))
                    }
                }
            }
            *last
          })
          .fold(HashMap::new(), |mut map, (c, cnt)| {
            (1..=cnt)
              .for_each(|i| {
                let entry = map.entry((c, i)).or_insert(0);
                *entry += 1;
              });
            map
          });
        freq
          .into_iter()
          .filter(|(_, v)| *v >= 3)
          .map(|((_, cnt), _)| cnt)
          .max()
          .unwrap_or(-1)
    }
}
