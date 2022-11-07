use colored::Colorize;
use std::fmt::Display;

use crate::stone::Camp::{self, Down, Up};
use crate::stone::Stone;
use crate::stone::StoneType::{self, Bishop, Cannon, King, Knight, Mandarin, Pawn, Rook};

#[derive(Clone, Copy)]
enum StoneIndex {
    Alive(usize, usize),
    Dead(usize, usize),
}

#[derive(Clone)]
pub struct StoneMap {
    stone_map: [[Option<Stone>; 9]; 10],
    up_stones: [StoneIndex; 16],
    down_stones: [StoneIndex; 16],
    turn: Camp,
}

use crate::step::Step;
use StoneIndex::Alive;

impl StoneMap {
    // public
    pub fn new() -> Self {
        // 16个上方棋子
        let mut stone_map: [[Option<Stone>; 9]; 10] = [[None; 9]; 10];
        let up_stones = [
            Alive(0, 0),
            Alive(0, 1),
            Alive(0, 2),
            Alive(0, 3),
            Alive(0, 4),
            Alive(0, 5),
            Alive(0, 6),
            Alive(0, 7),
            Alive(0, 8),
            Alive(3, 8),
            Alive(2, 7),
            Alive(3, 6),
            Alive(3, 4),
            Alive(3, 2),
            Alive(2, 1),
            Alive(3, 0),
        ];
        let down_stones = [
            Alive(9, 8),
            Alive(9, 7),
            Alive(9, 6),
            Alive(9, 5),
            Alive(9, 4),
            Alive(9, 3),
            Alive(9, 2),
            Alive(9, 1),
            Alive(9, 0),
            Alive(6, 0),
            Alive(7, 1),
            Alive(6, 2),
            Alive(6, 4),
            Alive(6, 6),
            Alive(7, 7),
            Alive(6, 8),
        ];

        let stone_types = [
            Rook, Knight, Bishop, Mandarin, King, Mandarin, Bishop, Knight, Rook, Pawn, Cannon,
            Pawn, Pawn, Pawn, Cannon, Pawn,
        ];

        // 将棋子摆放到棋盘上。
        for i in 0..stone_types.len() {
            if let Alive(x, y) = up_stones[i] {
                stone_map[x][y] = Some(Stone::new(stone_types[i], Up, i));
            }
        }

        for i in 0..stone_types.len() {
            if let Alive(x, y) = down_stones[i] {
                stone_map[x][y] = Some(Stone::new(stone_types[i], Down, i));
            }
        }

        return StoneMap {
            stone_map,
            up_stones,
            down_stones,
            turn: Camp::Down,
        };
    }

    pub fn switch_turn(&mut self) {
        self.turn = match self.turn {
            Camp::Up => Camp::Down,
            Camp::Down => Camp::Up,
        }
    }

    pub fn evaluate(&self) -> i32 {
        todo!()
    }

    pub fn make_move(&mut self, step: &Step) {
        let mover = self.stone_map[step.from.0][step.from.1];
        assert!(mover != None);
        assert!(self.stone_map[step.to.0][step.to.1] == step.killed);
        self.stone_map[step.from.0][step.from.1] = None;
        self.stone_map[step.to.0][step.to.1] = mover;
    }

    pub fn revoke_move(&mut self, step: &Step) {
        let mover = self.stone_map[step.to.0][step.to.1];
        assert!(mover != None);
        assert!(self.stone_map[step.from.0][step.from.1] == None);
        self.stone_map[step.from.0][step.from.1] = mover;
        self.stone_map[step.to.0][step.to.1] = step.killed;
    }

    pub fn can_move(&mut self, step: &Step) {
        todo!()
    }

    // 走法生成器
    pub fn generate_stone_steps(&mut self) -> Vec<Step> {
        todo!()
    }
    // private
    fn is_king_meeted(&mut self) -> bool {
        true
    }
    fn can_king_move(&mut self, from: (usize, usize), to: (usize, usize)) -> bool {
        todo!()
    }
    fn can_mandarin_move(&mut self, from: (usize, usize), to: (usize, usize)) -> bool {
        todo!()
    }
    fn can_bishop_move(&mut self, from: (usize, usize), to: (usize, usize)) -> bool {
        todo!()
    }
    fn can_knight_move(&mut self, from: (usize, usize), to: (usize, usize)) -> bool {
        todo!()
    }
    fn can_rook_move(&mut self, from: (usize, usize), to: (usize, usize)) -> bool {
        todo!()
    }
    fn can_cannon_move(&mut self, from: (usize, usize), to: (usize, usize)) -> bool {
        todo!()
    }
    fn can_pawn_move(&mut self, from: (usize, usize), to: (usize, usize)) -> bool {
        todo!()
    }
}

// 輸出棋盤
impl Display for StoneMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "　１２３４５６７８９　").unwrap();
        writeln!(f, "　＿＿＿＿＿＿＿＿＿　").unwrap();

        for i in 0..10 {
            write!(f, "｜").unwrap();
            for j in 0..9 {
                if let Some(stone) = self.stone_map[i][j] {
                    (match stone.stone_type {
                        StoneType::King => match stone.camp {
                            Camp::Up => write!(f, "{}", "將".bright_white()),
                            Camp::Down => write!(f, "{}", "帥".bright_red()),
                        },
                        StoneType::Mandarin => match stone.camp {
                            Camp::Up => write!(f, "{}", "士".bright_white()),
                            Camp::Down => write!(f, "{}", "仕".bright_red()),
                        },
                        StoneType::Bishop => match stone.camp {
                            Camp::Up => write!(f, "{}", "象".bright_white()),
                            Camp::Down => write!(f, "{}", "相".bright_red()),
                        },
                        StoneType::Knight => match stone.camp {
                            Camp::Up => write!(f, "{}", "馬".bright_white()),
                            Camp::Down => write!(f, "{}", "傌".bright_red()),
                        },
                        StoneType::Rook => match stone.camp {
                            Camp::Up => write!(f, "{}", "車".bright_white()),
                            Camp::Down => write!(f, "{}", "俥".bright_red()),
                        },
                        StoneType::Cannon => match stone.camp {
                            Camp::Up => write!(f, "{}", "砲".bright_white()),
                            Camp::Down => write!(f, "{}", "炮".bright_red()),
                        },
                        StoneType::Pawn => match stone.camp {
                            Camp::Up => write!(f, "{}", "卒".bright_white()),
                            Camp::Down => write!(f, "{}", "兵".bright_red()),
                        },
                    })
                    .unwrap();
                } else {
                    // 输出一个空格
                    let c = if i == 4 || i == 5 { '－' } else { '　' };
                    write!(f, "{}", c).unwrap();
                }
            }
            writeln!(f, "｜").unwrap();
        }

        writeln!(f, "　￣￣￣￣￣￣￣￣￣　").unwrap();
        writeln!(f, "　９８７６５４３２１　").unwrap();

        Ok(())
    }
}
