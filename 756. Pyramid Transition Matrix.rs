const VAL_BASE    : usize = (b'F' - b'A' + 1) as usize;
const VAL_MAX     : usize = VAL_BASE.pow(ROW_MAX_LEN as u32 - 1);
const ROW_MAX_LEN : usize = 6;
const A           : u8 = b'A';
impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let m = bottom.len();
        let n = allowed.len();
        let mut allowed_map = [[0_u8; VAL_BASE]; VAL_BASE];
        let mut seen        = [false; VAL_MAX];
        let mut rows        = [[0; ROW_MAX_LEN]; ROW_MAX_LEN];
        for a in allowed {
            let b = a.as_bytes();

            allowed_map[(b[0] - A) as usize][(b[1] - A) as usize] 
                |= 1 << b[2] - A;
        }
        for (i, b) in bottom.bytes().enumerate() {
            rows[m - 1][i] = b - A;
        }
        recurse(&mut rows, 0, m - 1, 0, &allowed_map, &mut seen)
    }
}
fn recurse(rows        : &mut [[u8; ROW_MAX_LEN]], 
           pattern     : usize,
           row_i       : usize,
           block_i     : usize,
           allowed_map : &[[u8; VAL_BASE]], 
           seen        : &mut [bool])
    -> bool
{
    if row_i == 1 && block_i == 1 {
        true
    } else if block_i == row_i {
        if seen[pattern] {
            false
        } else {
            seen[pattern] = true;
            recurse(rows, 0, row_i - 1, 0, allowed_map, seen)
        }
    } else {
        let ll_idx  = rows[row_i][block_i] as usize;
        let lr_idx  = rows[row_i][block_i + 1] as usize;
        let allowed = allowed_map[ll_idx][lr_idx] as usize;
        for b in (0..6) {
            if allowed & 1 << b != 0 {
                rows[row_i - 1][block_i] = b as u8;
                if recurse(rows, 
                           pattern * VAL_BASE + b, 
                           row_i, 
                           block_i + 1, 
                           allowed_map, 
                           seen) {
                    return true;
                }
            }
        }
        false
    }
}
