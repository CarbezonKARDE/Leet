impl Solution {
    pub fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
        use std::collections::VecDeque;
        let grid: Vec<Vec<u8>> = classroom.iter().map(|row| row.as_bytes().to_vec()).collect();
        let rows = grid.len();
        let cols = grid[0].len();
        let cells = rows * cols;
        let mut start = 0usize;
        let mut litter_bits = vec![0u16; cells];
        let mut litter_count = 0usize;
        for r in 0..rows {
            for c in 0..cols {
                let pos = r * cols + c;
                match grid[r][c] {
                    b'S' => start = pos,
                    b'L' => {
                        litter_bits[pos] = 1u16 << litter_count;
                        litter_count += 1;
                    }
                    _ => {}
                }
            }
        }
        if litter_count == 0 {
            return 0;
        }
        let mask_count = 1usize << litter_count;
        let goal = (mask_count - 1) as u16;
        let max_energy = energy as u8;
        let mut best_energy = vec![-1i8; cells * mask_count];
        let mut queue = VecDeque::new();
        best_energy[start * mask_count] = energy as i8;
        queue.push_back((start, 0u16, max_energy));
        let directions = [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)];
        let mut moves = 0;
        while !queue.is_empty() {
            let level_size = queue.len();
            for _ in 0..level_size {
                let (pos, mask, remaining) = queue.pop_front().unwrap();
                if mask == goal {
                    return moves;
                }
                if remaining == 0 {
                    continue;
                }
                let r = pos / cols;
                let c = pos % cols;
                for &(dr, dc) in &directions {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                        continue;
                    }
                    let nr = nr as usize;
                    let nc = nc as usize;
                    if grid[nr][nc] == b'X' {
                        continue;
                    }
                    let next_pos = nr * cols + nc;
                    let next_mask = mask | litter_bits[next_pos];
                    let next_energy = if grid[nr][nc] == b'R' {
                        max_energy
                    } else {
                        remaining - 1
                    };
                    if next_mask == goal {
                        return moves + 1;
                    }
                    let index = next_pos * mask_count + next_mask as usize;
                    if next_energy as i8 > best_energy[index] {
                        best_energy[index] = next_energy as i8;
                        queue.push_back((next_pos, next_mask, next_energy));
                    }
                }
            }
            moves += 1;
        }
        -1
    }
}
