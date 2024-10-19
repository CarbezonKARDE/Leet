impl Solution {
	pub fn find_kth_bit(n: i32, k: i32) -> char {
		let mut s = vec![b'0'];
		for _ in 1..n {
			s.push(b'1');
			for i in (0..s.len() - 1).rev() {
				s.push(b'0' + 1 ^ (s[i] - b'0'));
			}
		}
		s[k as usize - 1] as _
	}
}
