use std::collections::HashSet;
impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        if arr.is_empty() {
            return 0;
        }
        let mut all_values: Vec<i32> = Vec::with_capacity(arr.len() * 30);
        let mut current: [i32; 32] = [0; 32];
        let mut current_len = 0u8;
        unsafe {
            for &x in arr.iter() {
                let mut next: [i32; 32] = [0; 32];
                let mut next_len = 1u8;
                *next.get_unchecked_mut(0) = x;
                for i in 0..current_len as usize {
                    let new_or = x | *current.get_unchecked(i);
                    let mut found = new_or == x;
                    if !found {
                        for j in 1..next_len as usize {
                            if *next.get_unchecked(j) == new_or {
                                found = true;
                                break;
                            }
                        }
                    }
                    if !found && (next_len as usize) < 31 {
                        *next.get_unchecked_mut(next_len as usize) = new_or;
                        next_len += 1;
                    }
                }
                std::ptr::copy_nonoverlapping(next.as_ptr(), current.as_mut_ptr(), next_len as usize);
                current_len = next_len;
                for i in 0..current_len as usize {
                    all_values.push(*current.get_unchecked(i));
                }
            }
        }
        all_values.sort_unstable();
        all_values.dedup();
        all_values.len() as i32
    }
}
