use crate::SBoard;
use rand::seq::SliceRandom;

pub struct WFC{
    pub board:[[u8;9];9],
    pattern:[[[u8;9];9];9],
}

impl WFC{
    pub fn run(&mut self){
        let (i,j)=self.min_entropy_find();
        println!("Debug ({},{})",i,j);
        let val = self.collapse(i,j);
        self.propogate(i,j,val);
    }

    fn min_entropy_find(&self)->(usize,usize){
        let (mut i,mut j) =(0,0);
        let mut min = self.board[i][j];
        for l in 0..9{
            for k in 0..9{
                let t = self.board[l][k];
                if t<min{
                    min =t;
                    (i,j) = (l,k);
                }
            }
        }

        (i,j)
    }

    fn collapse(&mut self,i:usize,j:usize)->usize{
        let mut t= Vec::new();
        for k in 0..9{
            let t2 = self.pattern[i][j][k];
            if t2!=0{
                t.push(k);
            }
        }

        let mut val = t.choose(&mut rand::thread_rng()).expect("No possible state");
        val = &t[0];
        self.board[i][j]=*val as u8 + 1;
        println!("Debug-{}{}{}",i,j,val);
        self.pattern[i][j][*val]=0;
        val+1
    }

    fn propogate(&mut self,i:usize,j:usize,val:usize){
        for k in 0..9{
            self.pattern[i][k][val-1]=0;
            self.pattern[k][j][val-1]=0;
        }

        let x = match i{
            0..=2=>0,
            3..=5=>3,
            6..=8=>6,
            _=>panic!("something' wrong")
        };

        let y = match j{
            0..=2=>0,
            3..=5=>3,
            6..=8=>6,
            _=>panic!("something' wrong")
        };

        self.pattern[x][y][val-1]=0;
        self.pattern[x][y+1][val-1]=0;
        self.pattern[x][y+2][val-1]=0;
        self.pattern[x+1][y][val-1]=0;
        self.pattern[x+1][y+1][val-1]=0;
        self.pattern[x+1][y+2][val-1]=0;
        self.pattern[x+2][y][val-1]=0;
        self.pattern[x+2][y+1][val-1]=0;
        self.pattern[x+2][y+2][val-1]=0;

    }
}

impl WFC{
    pub fn new()->Self{
        let mut pattern = [[[1;9];9];9];
        let mut board = [[0;9];9];

        Self{board,pattern}
    }

    pub fn load(&mut self,gboard:SBoard){
        for i in 0..9{
            for j in 0..9{
                let t = gboard.board[i][j] as usize;
                self.board[i][j]= t as u8;
                if t!=0{
                    self.pattern[i][j][t-1] = 0;
                    self.propogate(i, j, t)
                }
            }
        }
    }
}