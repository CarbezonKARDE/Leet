use std::collections::{HashMap, VecDeque};
impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let final_tiles = vec![1, 2, 3, 4, 5, 0];
        let final_id = Solution::board_id(&final_tiles);
        let final_zpos = 5;
        let mut start_tiles = Vec::new();
        for row in &board {
            start_tiles.extend(row);
        }
        let start_id = Solution::board_id(&start_tiles);
        let directions = vec![(0, -1), (0, 1), (1, 0), (-1, 0)];
        let mut queue = VecDeque::new();
        queue.push_back((final_id, final_zpos));
        let mut dist = HashMap::new();
        dist.insert(final_id, 0);
        while let Some((current_id, zpos)) = queue.pop_front() {
            let current_dist = dist[&current_id];
            let mut tiles = Solution::from_id(current_id);
            for &(di, dj) in &directions {
                if let Some((next_zpos, next_id)) = Solution::do_step(&mut tiles, zpos, di, dj) {
                    if !dist.contains_key(&next_id) {
                        dist.insert(next_id, current_dist + 1);
                        queue.push_back((next_id, next_zpos));
                    }
                    Solution::undo_step(&mut tiles, next_zpos, zpos);
                }
            }
        }
        dist.get(&start_id).cloned().unwrap_or(-1)
    }
    fn board_id(tiles: &[i32]) -> i32 {
        tiles.iter().fold(0, |id, &t| id * 6 + t)
    }
    fn from_id(mut id: i32) -> Vec<i32> {
        let mut tiles = vec![0; 6];
        for i in (0..6).rev() {
            tiles[i] = id % 6;
            id /= 6;
        }
        tiles
    }
    fn do_step(tiles: &mut Vec<i32>, zpos: usize, di: i32, dj: i32) -> Option<(usize, i32)> {
        let r = zpos as i32 / 3;
        let c = zpos as i32 % 3;
        let nr = r + di;
        let nc = c + dj;
        if nr >= 0 && nr < 2 && nc >= 0 && nc < 3 {
            let new_zpos = (nr * 3 + nc) as usize;
            tiles.swap(zpos, new_zpos);
            Some((new_zpos, Solution::board_id(tiles)))
        } else {
            None
        }
    }
    fn undo_step(tiles: &mut Vec<i32>, zpos: usize, prev_zpos: usize) {
        tiles.swap(zpos, prev_zpos);
    }
}
