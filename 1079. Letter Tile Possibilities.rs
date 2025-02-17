impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        fn prim(num: &[usize], sum: usize, facts: &[i32]) -> i32 {
            if sum == 0 { return 0; }
            let mut res = facts[sum];
            for n in num.iter().copied() {
                res /= facts[n];
            }
            res
        }
        fn run(num: &mut [usize], j: usize, sum: usize, facts: &[i32]) -> i32 {
            if j == num.len() { return prim(num, sum, facts); }
            let mut res = run(num, j+1, sum, facts);
            let z = num[j];
            for x in 1..=z {
                num[j] -= 1;
                res += run(num, j+1, sum - x, facts);
            }
            num[j] = z;
            res
        }
        let mut hm = [0; 26];
        let mut facts = vec![0; tiles.as_bytes().len() + 1];
        facts[0] = 1;
        for (i, c) in tiles.bytes().enumerate() {
            hm[(c - b'A') as usize] += 1;
            facts[i+1] = facts[i] * (i+1) as i32;
        }
        let mut num = [0; 26];
        let mut it = 0;
        for x in hm.into_iter() {
            if x > 0 {
                num[it] = x;
                it += 1;
            }
        }
        run(&mut num[..it], 0, tiles.as_bytes().len(), &facts)
    }
}
