impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let mut total_area: i64 = 0;
        let mut events: Vec<(i32, i32, i32)> = Vec::new();
        for sq in &squares {
            let y = sq[1];
            let l = sq[2];
            total_area += l as i64 * l as i64;
            events.push((y, l, 1));
            events.push((y + l, l, -1));
        }
        events.sort_by_key(|&(y, _, _)| y);
        let mut covered_width: f64 = 0.0;
        let mut curr_area: f64 = 0.0;
        let mut prev_height: f64 = 0.0;
        for (y, l, delta) in events {
            let diff = y as f64 - prev_height;
            let area = covered_width * diff;
            if 2.0 * (curr_area + area) >= total_area as f64 {
                return prev_height + (total_area as f64 - 2.0 * curr_area) / (2.0 * covered_width);
            }
            covered_width += (delta * l) as f64;
            curr_area += area;
            prev_height = y as f64;
        }
        
        0.0
    }
}
