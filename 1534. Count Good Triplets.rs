impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut res = 0;
        for i in 0..arr.len() {
            let ai = arr[i];
            for j in i+1..arr.len() {
                let aj = arr[j];
                if (ai - aj).abs() > a { continue; }
                for k in j+1..arr.len() {
                    let ak = arr[k];
                    if (aj - ak).abs() <= b && (ai - ak).abs() <= c {
                        res += 1;
                    }
                }
            }
        }
        res
    }
}
