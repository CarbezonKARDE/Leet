const SIZE: usize = 26;
const MODULO: u64 = 1000000007;
pub fn square(poly: &mut [u32; SIZE]) {
    let mut intermediate: [u32; 2 * SIZE - 1] = [0; 2 * SIZE - 1];
    for (i, x) in intermediate.iter_mut().enumerate() {
        let min = i.saturating_sub(SIZE - 1);
        let max = i.min(SIZE - 1);
        let count = (max + 1 - min) / 2;
        let mut sum = 0u64;
        for j in min..(min + count) {
            sum += poly[j] as u64 * poly[i - j] as u64;
        }
        sum %= MODULO;
        if i % 2 == 0 {
            let half = poly[i / 2] as u64;
            sum = (half * half + 2 * sum) % MODULO;
        } else {
            sum = 2 * sum % MODULO;
        }
        *x = sum as u32;
    }
    let high_part = &intermediate[SIZE..];
    for (i, x) in poly.iter_mut().enumerate() {
        *x = (intermediate[i]
            + high_part.get(i.wrapping_sub(1)).copied().unwrap_or(0)
            + high_part.get(i).copied().unwrap_or(0))
            % MODULO as u32;
    }
}
pub fn shift(poly: &mut [u32; SIZE]) {
    let last = *poly.last().unwrap();
    poly.copy_within(..(SIZE - 1), 1);
    poly[0] = last;
    poly[1] += last;
}
pub fn coeff_sum(a: &[u32; SIZE]) -> u32 {
    (a.iter().map(|x| *x as u64).sum::<u64>() % MODULO) as u32
}
pub fn sum_prod_coeff(a: &[u32; SIZE], b: &[u32; SIZE]) -> u32 {
    let mut sum = 0u32;
    let mut a_coeffs = coeff_sum(a);
    for (bi, aj) in b.iter().zip(a.iter().rev()) {
        sum = ((sum as u64 + a_coeffs as u64 * *bi as u64) % MODULO) as u32;
        a_coeffs = (a_coeffs + aj) % MODULO as u32;
    }
    sum
}
pub fn shift_exp_sum(poly: &[u32; SIZE], t: u32) -> u32 {
    let mut t = t as i32;
    let mut x = [0; SIZE];
    let lzcnt = t.leading_zeros();
    t <<= lzcnt;
    x[1] = 1;
    for _ in 0..(31 - lzcnt) {
        square(&mut x);
        t <<= 1;
        if t < 0 {
            shift(&mut x);
        }
    }
    sum_prod_coeff(&x, poly)
}
pub fn count_string(s: &str, poly: &mut [u32; SIZE]) {
    for c in s.chars() {
        poly[(c as u32 - 'a' as u32) as usize] += 1;
    }
}
impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        let mut poly = [0; SIZE];
        count_string(&s, &mut poly);
        shift_exp_sum(&poly, t as u32) as i32
    }
}
