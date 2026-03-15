struct Fancy {
    sequence: Vec<i64>,
    add: i64,
    mult: i64,
    mod_val: i64,
}
impl Fancy {
    fn new() -> Self {
        Fancy {
            sequence: Vec::new(),
            add: 0,
            mult: 1,
            mod_val: 1000000007,
        }
    }
    fn append(&mut self, val: i32) {
        let val = ((val as i64 - self.add) * mod_inv(self.mult, self.mod_val)) % self.mod_val;
        self.sequence.push(val);
    }
    fn add_all(&mut self, inc: i32) {
        self.add = (self.add + inc as i64) % self.mod_val;
    }
    fn mult_all(&mut self, m: i32) {
        self.mult = (self.mult * (m as i64)) % self.mod_val;
        self.add = (self.add * (m as i64)) % self.mod_val;
    }
    fn get_index(&self, idx: i32) -> i32 {
        if (idx as usize) < self.sequence.len() {
            let val = (self.sequence[idx as usize] * self.mult + self.add) % self.mod_val;
            (if val < 0 {
                val + self.mod_val
            } else {
                val
            }) as i32
        } else {
            -1
        }
    }
}
fn mod_inv(a: i64, m: i64) -> i64 {
    let mut a = (a % m + m) % m;
    let mut b = m;
    let mut x = 0i64;
    let mut y = 1i64;
    while a != 0 {
        let q = b / a;
        let tmp = b % a;
        b = a;
        a = tmp;

        let tmp_x = x - q * y;
        x = y;
        y = tmp_x;
    }
    if x < 0 {
        x += m;
    }
    x
}
