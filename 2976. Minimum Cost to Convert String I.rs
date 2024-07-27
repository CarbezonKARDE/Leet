impl Solution {
  pub fn minimum_cost(source: String, target: String, original: Vec<char>, changed: Vec<char>, cost: Vec<i32>) -> i64 {
    let mut C = vec![vec![i32::MAX as u64; 26]; 26];
    for i in 0 .. original.len() {
      let c1 = original[i] as usize - 97;
      let c2 = changed[i] as usize - 97;
      C[c1][c2] = C[c1][c2].min(cost[i] as u64);
    }
    for k in 0 .. 26 {
      for i in 0 .. 26 {
        for j in 0 .. 26 {
          C[i][j] = C[i][j].min(C[i][k] + C[k][j])
        }
      }
    }
    let mut res = 0;
    let source = source.into_bytes();
    let target = target.into_bytes();
    for i in 0 .. source.len() {
      if target[i] != source[i] {
        let v = C[source[i] as usize - 97][target[i] as usize - 97];
        if v >= i32::MAX as u64 {
          return -1;
        }
        res += v;
      }
    }
    return res as i64;
  }
}
