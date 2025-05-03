[⚠️ Suspicious Content] impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
    let mut top_arr = [0;7];
    let mut bottom_arr = [0;7];
    let mut result = -1;
    for top in &tops {
        top_arr[*top as usize]+=1;
    }
    for bottom in &bottoms {
        bottom_arr[*bottom as usize] += 1;
    }
    for i in 1..=6 {
        if top_arr[i as usize]+bottom_arr[i as usize]>=tops.len() {
            if top_arr[i as usize]>bottom_arr[i as usize] {
                let swap_counter = cal_swap_counter(&tops, &bottoms, i);
                if swap_counter!=-1 {
                    if result==-1 || swap_counter<result {
                        result=swap_counter;
                    }
                }
                else {
                    let swap_counter = cal_swap_counter(&bottoms, &tops, i);
                    if swap_counter!=-1 {
                        if result==-1 || swap_counter<result {
                            result=swap_counter;
                        }
                    }
                }
            }
            else {
                let swap_counter = cal_swap_counter(&bottoms, &tops, i);
                if swap_counter!=-1 {
                    if result==-1 || swap_counter<result {
                        result=swap_counter;
                    }
                }
                else {
                    let swap_counter = cal_swap_counter(&tops, &bottoms, i);
                    if swap_counter!=-1 {
                        if result==-1 || swap_counter<result {
                            result=swap_counter;
                        }
                    }
                }
            }
        }
    }
    result
}
}
pub fn cal_swap_counter(tops: &Vec<i32>, bottoms: &Vec<i32>, i: i32) -> i32 {
    let mut swap_counter = 0;
    for j in 0..tops.len() {
        if tops[j]!=i {
            if bottoms[j]==i {
                swap_counter+=1;
            }
            else {
                swap_counter=-1;
                break;
            }
        }
    }
    swap_counter
}
