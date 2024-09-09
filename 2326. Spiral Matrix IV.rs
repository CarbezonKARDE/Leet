enum Direction{
    Right = 0,
    Down,
    Left,
    Up
}
impl Direction{
    fn turn(self) -> Direction{
        match (self as i32 + 1)%4{
            0 => Direction::Right,
            1 => Direction::Down, 
            2 => Direction::Left,
            3 => Direction::Up,
            _ => unreachable!()
        }
    }
}
impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let (m, n, mut x, mut y) = (m as usize, n as usize, 0usize, 0usize);
        let mut grid = vec![vec![-1; n]; m]; 
        let mut dir = Direction::Right;
        let mut current = head;
        let mut steps = 0;
        while let Some(node) = current {
            grid[x][y] = node.val;
            current = node.next;
            steps += 1;
            match dir {
                Direction::Right => {
                    if y + 1 == n || grid[x][y + 1] != -1 {
                        dir = dir.turn();
                    }
                }
                Direction::Down => {
                    if x + 1 == m || grid[x + 1][y] != -1 {
                        dir = dir.turn();
                    }
                }
                Direction::Left => {
                    if y == 0 || grid[x][y - 1] != -1 {
                        dir = dir.turn();
                    }
                }
                Direction::Up => {
                    if x == 0 || grid[x - 1][y] != -1 {
                        dir = dir.turn();
                    }
                }
            }
            match dir {
                Direction::Right => y += 1,
                Direction::Down => x += 1,
                Direction::Left => y = y.saturating_sub(1),
                Direction::Up => x = x.saturating_sub(1),
            }
        }
        grid
    }
}
