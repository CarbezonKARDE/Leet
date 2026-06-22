impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let v1 = [1, 1, 2, 2, 1];
        let mut v2 = [0;5];
        for i in text.chars() {
            match i {
                'b' => v2[0] += 1,
                'a' => v2[1] += 1,
                'l' => v2[2] += 1,
                'o' => v2[3] += 1,
                'n' => v2[4] += 1,
                _ => continue
            }
        }
        (0..v1.len()).map(|i| v2[i] / v1[i]).min().unwrap()
    }
}
