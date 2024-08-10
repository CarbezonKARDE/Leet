impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let w = grid.len();
        let mut parent: Vec<usize> = (0..w * w * 4).collect();
        let mut size = vec![1; w * w * 4];
        for (y, s) in grid.iter().enumerate() {
            for (x, b) in s.bytes().enumerate() {
                let i = (y * w + x) * 4;
                if x + 1 < w {
                    Self::union_by_size(i, i + 6, &mut parent, &mut size);
                }
                if y + 1 < w {
                    Self::union_by_size(i + 1, i + w * 4 + 3, &mut parent, &mut size);
                }
                if b != b'/' {
                    Self::union_by_size(i, i + 3, &mut parent, &mut size);
                    Self::union_by_size(i + 1, i + 2, &mut parent, &mut size);
                }
                if b != b'\\' {
                    Self::union_by_size(i, i + 1, &mut parent, &mut size);
                    Self::union_by_size(i + 2, i + 3, &mut parent, &mut size);
                }
            }
        }
        size.iter().filter(|&&c| c != 0).count() as i32
    }
    fn find_set(i: usize, parent: &mut Vec<usize>) -> usize {
        if parent[i] != i {
            parent[i] = Self::find_set(parent[i], parent);
        }
        parent[i]
    }
    fn union_by_size(i: usize, j: usize, parent: &mut Vec<usize>, size: &mut Vec<i32>) {
        let mut i_root = Self::find_set(i, parent);
        let mut j_root = Self::find_set(j, parent);
        if i_root != j_root {
            if size[i_root] < size[j_root] {
                std::mem::swap(&mut i_root, &mut j_root);
            }
            parent[j_root] = i_root;
            size[i_root] += size[j_root];
            size[j_root] = 0;
        }
    }
}
