impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        if k == 1 {
            return nums.len() as i32;
        }
        let mut remainers: Vec<Vec<usize>> = Vec::with_capacity(k as usize);
        for _ in 0..k {
            remainers.push(vec![]);
        }
        for i in 0..nums.len() {
            remainers[(nums[i] % k) as usize].push(i);
        }
        remainers.sort_by(|a, b| a.len().cmp(&b.len()));
        let mut first_not_empty_index = 0_usize;
        while remainers[first_not_empty_index].len() == 0 {
            first_not_empty_index += 1;
        }
        remainers.splice(0..first_not_empty_index, []);
        remainers.reverse();
        let mut max_result = 0_i32;
        if remainers.len() == 1 {
            return remainers[0].len() as i32;
        }
        for i in 0..(remainers.len() - 1) {
            if remainers[i].len() + remainers[i + 1].len() <= max_result as usize {
                break;
            }
            for j in (i + 1)..remainers.len() {
                let mut i_pointer = 0_usize;
                let mut j_pointer = 0_usize;
                let mut i_greater = remainers[i][0] > remainers[j][0];
                let mut zebra_length = 2;
                while i_pointer < remainers[i].len() && j_pointer < remainers[j].len() {
                    if remainers[i][i_pointer] > remainers[j][j_pointer] {
                        if !i_greater {
                            zebra_length += 1;
                            i_greater = true;
                        }
                        j_pointer += 1;
                    } else {
                        if i_greater {
                            zebra_length += 1;
                            i_greater = false;
                        }
                        i_pointer += 1;
                    }
                }
                if i_pointer == remainers[i].len()
                    && remainers[i][i_pointer - 1] > remainers[j][j_pointer]
                {
                    while j_pointer < remainers[j].len() {
                        if remainers[j][j_pointer] > remainers[i][i_pointer - 1] {
                            zebra_length += 1;
                        }
                        j_pointer += 1;
                    }
                }
                if j_pointer == remainers[j].len()
                    && remainers[j][j_pointer - 1] > remainers[i][i_pointer]
                {
                    while i_pointer < remainers[i].len() {
                        if remainers[i][i_pointer] > remainers[j][j_pointer - 1] {
                            zebra_length += 1;
                        }
                        i_pointer += 1;
                    }
                }
                let local_result = zebra_length
                    .max(remainers[i].len() as i32)
                    .max(remainers[j].len() as i32);
                if local_result > max_result {
                    max_result = local_result
                }
            }
        }
        max_result
    }
}
