impl Solution {
    pub fn get_happy_string(mut n: i32, mut k: i32) -> String {
        k -= 1;
        let mut num = 3 * 2_i32.pow(n as u32 - 1);
        if k >= num { return String::new(); }
        let mut ans = String::new();
        let next = [
            ['b', 'c'],
            ['a', 'c'],
            ['a', 'b']
        ];
        let mut cur = usize::MAX;
        cur = (3 * k / num) as usize;
        match cur {
            0 => ans.push('a'),
            1 => ans.push('b'),
            2 => ans.push('c'),
            _ => unreachable!(),
        };
        n -= 1;
        while n > 0 {
            let modulus = 2_i32.pow(n as u32);
            k = (k % modulus + modulus) % modulus;
            let c = next[cur][(2 * k / modulus) as usize];
            ans.push(c);
            cur = match c {
                'a' => 0,
                'b' => 1,
                'c' => 2,
                _ => unreachable!(),
            };
            n -= 1;
        }
        ans
    }
}
