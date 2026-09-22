#[derive(Clone)]
struct Node {
    cnt: Vec<i32>,
    prod: i32,
}
impl Node {
    fn new(k: usize) -> Self {
        Self { cnt: vec![0; k], prod: 1 }
    }
}
struct SegTree {
    k: i32,
    n: usize,
    s: usize,
    tree: Vec<Node>,
}
impl SegTree {
    fn new(nums: &[i32], k: i32) -> Self {
        let n = nums.len();
        let mut s = 1;
        while s < n {
            s <<= 1;
        }
        let mut tree = vec![Node::new(k as usize); 2 * s];
        for i in 0..n {
            let a_mod = nums[i] % k;
            tree[s + i].cnt[a_mod as usize] = 1;
            tree[s + i].prod = a_mod;
        }
        for p in (1..s).rev() {
            tree[p] = Self::merge(&tree[2 * p], &tree[2 * p + 1]);
        }

        Self { k, n, s, tree }
    }
    fn merge(l: &Node, r: &Node) -> Node {
        let mut res = Node::new(l.cnt.len());
        res.prod = (l.prod * r.prod) % l.cnt.len() as i32;
        for (i, &c) in l.cnt.iter().enumerate() {
            res.cnt[i] += c;
        }
        for r_b in 0..l.cnt.len() {
            let c = r.cnt[r_b];
            if c > 0 {
                let r_ = (l.prod * r_b as i32) % l.cnt.len() as i32;
                res.cnt[r_ as usize] += c;
            }
        }
        res
    }
    fn update(&mut self, idx: usize, val: i32) {
        let pos = self.s + idx;
        let a_mod = val % self.k;
        self.tree[pos].cnt.fill(0);
        self.tree[pos].cnt[a_mod as usize] = 1;
        self.tree[pos].prod = a_mod;
        let mut pos = pos >> 1;
        while pos >= 1 {
            self.tree[pos] = Self::merge(&self.tree[2 * pos], &self.tree[2 * pos + 1]);
            pos >>= 1;
        }
    }
    fn query(&self, l: usize, r: usize) -> Node {
        let mut cnt_l = Node::new(self.k as usize);
        let mut cnt_r = Node::new(self.k as usize);
        cnt_l.prod = 1;
        cnt_r.prod = 1;
        let mut l = l + self.s;
        let mut r = r + self.s;
        while l < r {
            if l & 1 == 1 {
                cnt_l = Self::merge(&cnt_l, &self.tree[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                cnt_r = Self::merge(&self.tree[r], &cnt_r);
            }
            l >>= 1;
            r >>= 1;
        }
        Self::merge(&cnt_l, &cnt_r)
    }
}
impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut st = SegTree::new(&nums, k);
        let mut res = Vec::with_capacity(queries.len());
        for q in queries {
            let (idx, val, start, x) = (q[0], q[1], q[2], q[3]);
            st.update(idx as usize, val);
            let result = st.query(start as usize, nums.len());
            res.push(result.cnt[x as usize]);
        }
        res
    }
}
