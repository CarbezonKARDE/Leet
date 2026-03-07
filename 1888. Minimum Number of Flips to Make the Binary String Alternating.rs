impl Solution {
    pub fn min_flips(stri: String) -> i32 {
        if stri.len() & 1 == 0 {
            stri.bytes()
                .fold([0; 2], |[o, l], x| match x {
                    b'0' => [l, o + 1],
                    _ => [l + 1, o],
                })
                .into_iter()
                .min()
                .unwrap()
        } else {
            stri.bytes()
                .fold([0; 4], |[o, l, _0, _1], x| match x {
                    b'0' => [l, o + 1, _1.min(o), _0 + 1],
                    _ => [l + 1, o, _1 + 1, _0.min(l)],
                })
                .into_iter()
                .min()
                .unwrap()
        }
    }
}
