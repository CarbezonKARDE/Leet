impl Solution {
    pub fn maximum_energy(mut energy: Vec<i32>, k: i32) -> i32 {
        let mut max = i32::MIN;
        let k = k as usize;
        let n = energy.len();
        for i in n-k ..n {
            max = max.max(energy[i]);
        }
        for i in (0..n - k).rev() {
            energy[i] += energy[i + k];
            max = max.max(energy[i]);
        }
        max
    }
}
