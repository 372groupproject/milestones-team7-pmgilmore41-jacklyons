extern crate rand;


pub mod connect4 {
    use rand::Rng;
    pub struct Connect4Board{
        board: [[i16; 7] ; 6],
        freespots: i16,
    }

    impl Connect4Board {
        pub fn new () -> Connect4Board {
            Connect4Board {
                board: [[0;7] ; 6],
                freespots: 6*7,
            }
        }

        pub fn isfull(self) ->bool{
           let ret = if self.freespots == 0 {
               true
           }
           else {
                false
           };

           ret
        }
        pub fn display(self) {
            println!("[0][1][2][3][4][5][6]");
            for row in self.board.iter(){
                for col in row.iter(){
                    print!(" {} ", col);
                }
                println!();
            }
        }

        #[allow(unused_assignments)]
        pub fn compdrop(&mut self) -> usize{
            let mut avail = Vec::new();
            let row: usize = 0;
            let mut col: usize = 0;

            while col < 6 {
                if self.board[row][col] == 0 {
                    avail.push(col);
                }
                col += 1;
            }
            
            let mut rng = rand::thread_rng();
            let index: usize = rng.gen::<usize>() % (avail.len() as usize);

            index
        }
        pub fn drop(&mut self, player: i16, col: usize) -> i32 {
            let mut i: usize = 0;
            let mut bad = 0;
            while i < 6 {
                let ret = if self.board[i][col] != 0 {
                    if i == 0 {
                        bad = -1;
                        -1
                    }
                    else {
                        self.board[i-1][col] = player;
                        1
                    }
                }
                else if i == 5 {
                    self.board[i][col] = player;
                    1
                }
                else {
                    i += 1;
                    0
                };

                if ret != 0 {
                    self.freespots = self.freespots - 1;
                    break;
                }
            }

            let win = if bad == -1 {
                -1
            }
            else if Connect4Board::is_win(self.board, player) {
                1
            }
            else {
                0
            };
            win
        }

        fn is_win(board: [[i16;7] ; 6], player: i16) -> bool {
            let mut win = false;
            let mut i = 0;
            let mut j = 0;

            while (i < 6) & !win {
                while j < 7 {
                    if j >= 3 {
                        win = Connect4Board::fourrow(board, i as usize, (j-3) as usize, i as usize, j as usize, player);
                        if win {
                            break;
                        }
                    }
                    if j <= 3 {
                        win = Connect4Board::fourrow(board, i as usize, j as usize, i as usize, (j+3) as usize, player);
                        if win {
                            break;
                        }
                    }
                    if i <= 2 {
                        win = Connect4Board::fourrow(board, i as usize, j as usize, (i+3) as usize, j as usize, player);
                        if win {
                            break;
                        }
                    }
                    if i >= 3 {
                        win = Connect4Board::fourrow(board, (i-3) as usize, j as usize, i as usize, j as usize, player);
                        if win {
                            break;
                        }
                    }
                    if (i >= 3) & (j <= 3) {
                        win = Connect4Board::fourrow(board, (i-3) as usize, j as usize, i as usize, (j+3) as usize, player);
                        if win {
                            break;
                        }
                    }
                    if (i <= 2) & (j <= 3) {
                        win = Connect4Board::fourrow(board, i as usize, j as usize, i+3 as usize, (j+3) as usize, player);
                        if win {
                            break;
                        }
                    }
                    j += 1;
                }
                i += 1;
                j = 0;
            }
            win
        }

        fn fourrow(temp: [[i16;7] ; 6], mut startrow:  usize, mut startcol: usize, endrow: usize, endcol: usize, player: i16) -> bool{
            let ret = loop {
                if temp[startrow][startcol] != player {
                    break false;
                }
                
                if (startrow == endrow) & (startcol == endcol) {
                    break true;
                }

                if startcol != endcol {
                    startcol += 1;
                }
                if startrow != endrow {
                    startrow += 1;
                }  
            };
            ret
        }
    }

    impl Copy for Connect4Board {}

    #[allow(non_snake_case)]
    impl Clone for Connect4Board {
        fn clone(&self) -> Connect4Board {
            Connect4Board{board: self.board, freespots: self.freespots}
        }
    }
}
