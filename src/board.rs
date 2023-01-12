use std::collections::HashMap;
use std::io::Write;

pub struct SBoard {
    board: [[u8; 9]; 9],
}

impl SBoard {
    fn is_valid(&self) -> bool {
        for i in 0..9 as usize {
            if !self.chk_row(i) || !self.chk_col(i) || !self.chk_sqr(i) {
                return false;
            }
        }

        true
    }

    fn chk_row(&self, row: usize) -> bool {
        let mut digits = HashMap::new();
        for i in 0..9 as usize {
            let cur_dig = self.board[row][i];
            if cur_dig != 0 && digits.contains_key(&cur_dig) {
                return false;
            }
            digits.insert(cur_dig, true);
        }
        true
    }

    fn chk_col(&self, col: usize) -> bool {
        let mut digits = HashMap::new();
        for i in 0..9 as usize {
            let cur_dig = self.board[i][col];
            if cur_dig != 0 && digits.contains_key(&cur_dig) {
                return false;
            }
            digits.insert(cur_dig, true);
        }
        true
    }

    fn chk_sqr(&self, grid: usize) -> bool {
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
            _ => panic!("Illegal grid"),
        };

        let mut sqrs = Vec::new();

        let mut digits = HashMap::new();

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
            let cur_dig = self.board[r][c];
            if cur_dig != 0 && digits.contains_key(&cur_dig) {
                return false;
            }
            digits.insert(cur_dig, true);
        }

        true
    }
}

impl SBoard {
    pub fn new() -> Self {
        Self { board: [[0; 9]; 9] }
    }

    pub fn load() -> Self {
        let mut new_board = Self::new();

        print!("How many entries?: ");

        let entry_no = get_input();

        for _i in 0..entry_no {
            print!("Enter row and col:");
            let Tuple(r, c): Tuple = get_input();
            print!("Enter value: ");
            let num = get_input();
            new_board.board[r - 1][c - 1] = num;
        }
        new_board
    }
}

pub fn get_input<T: std::str::FromStr>() -> T {
    std::io::stdout().flush().unwrap();
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf);
    match buf.trim().parse() {
        Ok(n) => n,
        Err(_) => panic!("Wrong input"),
    }
}

struct Tuple(usize, usize);

impl std::str::FromStr for Tuple {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vals = s.split_whitespace();
        let a: usize = vals.nth(0).unwrap().parse().unwrap();
        let b: usize = vals.nth(0).unwrap().parse().unwrap();
        Ok(Tuple(a, b))
    }
}

impl SBoard {
    pub fn display(&self) {
        let b = self.board;
        for i in 0..9 as usize {
            for j in 0..9 as usize {
                match j {
                    3 | 6 => print!("|{}", self.board[i][j]),
                    _ => print!("{}", self.board[i][j]),
                }
            }
            match i {
                2 | 5 => print!("\n-----------\n"),
                _ => print!("\n"),
            }
        }
    }
}
