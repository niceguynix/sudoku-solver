use std::{collections::HashMap, hash::Hash};

struct SBoard {
    board: [[u8; 9]; 9],
}

impl SBoard {
    fn is_valid() -> bool {
        for i in 0..9 as usize {
            if !chk_row(i) || !chk_col(i) || !chk_sqr() {
                false
            }
        }

        true
    }

    fn chk_row(row: usize) -> bool {
        let digits = HashMap::new();
        for i in 0..9 as usize {
            cur_dig = board[row][i];
            if cur_dig != 0 && digits.contains_key(cur_dig) {
                false
            }
            digits.insert(cur_dig, true);
        }
    }

    fn chk_col(col: usize) -> bool {
        let digits = HashMap::new();
        for i in 0..9 as usize {
            cur_dig = board[i][col];
            if cur_dig != 0 && digits.contains_key(cur_dig) {
                false
            }
            digits.insert(cur_dig, true);
        }
    }

    fn chk_sqr(grid: usize) -> bool {
        let (i, j) = match grid {
            0 => (0, 0),
            1 => (0, 3),
            2 => (0, 6),
            3 => (3, 0),
            4 => (3, 3),
            5 => (3, 6),
            6 => (6, 0),
            7 => (6, 3),
            8 => (6, 6),
        };

        let sqrs = Vec::new();

        let digits = HashMap::new();

        sqrs.push((i + 0, j + 0));
        sqrs.push((i + 0, j + 1));
        sqrs.push((i + 0, j + 2));
        sqrs.push((i + 1, j + 0));
        sqrs.push((i + 1, j + 1));
        sqrs.push((i + 1, j + 2));
        sqrs.push((i + 2, j + 0));
        sqrs.push((i + 2, j + 1));
        sqrs.push((i + 2, j + 2));

        for (r, c) in sqrs {
            cur_dig = board[r][c];
            if cur_dig != 0 && digits.contains_key(cur_dig) {
                false
            }
            digits.insert(cur_dig, true);
        }

        true
    }
}
