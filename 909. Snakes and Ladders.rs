use std::collections::{VecDeque};
impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        fn calc_pos(value: usize,n:usize) -> (usize,usize) {
            let row = value / n;
            let col = value - row*n;
            return ((n-1)-row,if row % 2 == 0 {col} else {(n-1)-col})
        }
        let mut queue:VecDeque<(i32,i32)> = VecDeque::from_iter(vec![(1,0)]);
        let n = board.len();
        let mut visited = vec![false;n*n+1];
        while let Some((value,dist)) = queue.pop_front() {
            if value as usize == n*n { return dist; }
            for idx in 1..=6 {
                if value as usize + idx > n*n {break;}
                let (row,col) = calc_pos(value as usize+idx-1,n);
                let dest = if board[row][col] == -1 {value+idx as i32} else {board[row][col]};
                if visited[dest as usize] { continue };
                queue.push_back((dest,dist+1));
                visited[dest as usize] = true;
            }
        }
        -1
    }
}
