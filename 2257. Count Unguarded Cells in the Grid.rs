impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut g = vec![vec![0; n]; m];        
        for e in guards.iter() {
            g[e[0] as usize][e[1] as usize] = 2;
        }
        for e in walls.iter() {
            g[e[0] as usize][e[1] as usize] = 2;
        }        
        let dirs = [-1, 0, 1, 0, -1];        
        for e in guards.iter() {
            for k in 0..4 {
                let mut x = e[0] as i32;
                let mut y = e[1] as i32;
                let dx = dirs[k];
                let dy = dirs[k + 1];                
                while x + dx >= 0 && x + dx < m as i32 && y + dy >= 0 && y + dy < n as i32 
                    && g[(x + dx) as usize][(y + dy) as usize] < 2 {
                    x += dx;
                    y += dy;
                    g[x as usize][y as usize] = 1;
                }
            }
        }        
        g.iter().flat_map(|row| row.iter())
            .filter(|&&cell| cell == 0)
            .count() as i32
    }
}
