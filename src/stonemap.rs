use colored::Colorize;
use std::fmt::Display;

use crate::stone::Camp;
use crate::stone::Stone;
use crate::stone::StoneType;

// #[derive(Clone, Copy)]
// enum StoneIndex {
//     Alive(usize, usize),
//     Dead(usize, usize),
// }

#[derive(Clone)]
pub struct StoneMap {
    stone_map: [[Option<Stone>; 9]; 10],
    // up_stones: [StoneIndex; 16],
    // down_stones: [StoneIndex; 16],
    turn: Camp
}

// use StoneIndex::Alive;
use crate::step::Step;

impl StoneMap {
    // public
    pub fn new() -> Self {
        // 定义一下32个棋子
        let king_up = Some(Stone::new(StoneType::King, Camp::Up));
        let king_down = Some(Stone::new(StoneType::King, Camp::Down));

        let mandarin_up = Some(Stone::new(StoneType::Mandarin, Camp::Up));
        let mandarin_down = Some(Stone::new(StoneType::Mandarin, Camp::Down));

        let bishop_up = Some(Stone::new(StoneType::Bishop, Camp::Up));
        let bishop_down = Some(Stone::new(StoneType::Bishop, Camp::Down));

        let knight_up = Some(Stone::new(StoneType::Knight, Camp::Up));
        let knight_down = Some(Stone::new(StoneType::Knight, Camp::Down));

        let rook_up = Some(Stone::new(StoneType::Rook, Camp::Up));
        let rook_down = Some(Stone::new(StoneType::Rook, Camp::Down));

        let cannon_up = Some(Stone::new(StoneType::Cannon, Camp::Up));
        let cannon_down = Some(Stone::new(StoneType::Cannon, Camp::Down));

        let pawn_up = Some(Stone::new(StoneType::Pawn, Camp::Up));
        let pawn_down = Some(Stone::new(StoneType::Pawn, Camp::Down));
        return StoneMap {
            stone_map: [
                [
                    rook_up,
                    knight_up,
                    bishop_up,
                    mandarin_up,
                    king_up,
                    mandarin_up,
                    bishop_up,
                    knight_up,
                    rook_up,
                ],
                [None; 9],
                [
                    None, cannon_up, None, None, None, None, None, cannon_up, None,
                ],
                [
                    pawn_up, None, pawn_up, None, pawn_up, None, pawn_up, None, pawn_up,
                ],
                [None; 9],
                [None; 9],
                [
                    pawn_down, None, pawn_down, None, pawn_down, None, pawn_down, None, pawn_down,
                ],
                [
                    None,
                    cannon_down,
                    None,
                    None,
                    None,
                    None,
                    None,
                    cannon_down,
                    None,
                ],
                [None; 9],
                [
                    rook_down,
                    knight_down,
                    bishop_down,
                    mandarin_down,
                    king_down,
                    mandarin_down,
                    bishop_down,
                    knight_down,
                    rook_down,
                ],
            ],
            // up_stones: [
            //     Alive(0, 0),
            //     Alive(0, 1),
            //     Alive(0, 2),
            //     Alive(0, 3),
            //     Alive(0, 4),
            //     Alive(0, 5),
            //     Alive(0, 6),
            //     Alive(0, 7),
            //     Alive(0, 8),
            //     Alive(3, 8),
            //     Alive(2, 7),
            //     Alive(3, 6),
            //     Alive(3, 4),
            //     Alive(3, 2),
            //     Alive(2, 1),
            //     Alive(3, 0),
            // ],
            // down_stones: [
            //     Alive(6, 0),
            //     Alive(7, 1),
            //     Alive(6, 2),
            //     Alive(6, 4),
            //     Alive(6, 6),
            //     Alive(7, 7),
            //     Alive(6, 8),
            //     Alive(9, 8),
            //     Alive(9, 7),
            //     Alive(9, 6),
            //     Alive(9, 5),
            //     Alive(9, 4),
            //     Alive(9, 3),
            //     Alive(9, 2),
            //     Alive(9, 1),
            //     Alive(9, 0),
            // ],
            turn: Camp::Down
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
                    }).unwrap();
                } else {
                    // 输出一个空格
                    let c = if i == 4 || i == 5 { '－' } else { '　' };
                    write!(f, "{}", c).unwrap();
                }
            }
            writeln!(f, "｜").unwrap();
        }

        writeln!(f, "　￣￣￣￣￣￣￣￣￣　").unwrap();
        writeln!(f, "　１２３４５６７８９　").unwrap();

        Ok(())
    }
}
