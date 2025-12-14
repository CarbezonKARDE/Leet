impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let bytes = corridor.as_bytes();
        let seat_count = bytes.iter().filter(|x| **x == b'S').count();
        if seat_count == 0 || (seat_count & 1) == 1 {
            return 0;
        }
        let mut curr_seat_count = 0_i64;
        let mut continues_plant_count = 0_i64;
        let len = bytes.len();
        const MOD_TO: i64 = 1_000_000_007;
        let mut start_index = 0_usize;
        let mut end_index = len - 1;
        let mut count = 1_i64;
        while bytes[start_index] != b'S' {
            start_index += 1;
        }
        while bytes[end_index] != b'S' {
            end_index -= 1;
        }
        for i in start_index..=end_index {
            if bytes[i] == b'S' {
                curr_seat_count += 1;
                if (curr_seat_count & 1) == 1 {
                    count = (count * (continues_plant_count + 1)) % MOD_TO;
                }
                continues_plant_count = 0;
            } else {
                continues_plant_count += 1;
            }
        }
        count as i32
    }
}
