use colored::Colorize;
use std::fmt::Display;

use crate::stone::Camp;
use crate::stone::Stone;
use crate::stone::StoneType;

#[derive(Clone, Copy)]
enum StoneIndex {
    Alive(i32, i32),
    Dead(i32, i32),
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
            up_stones: [
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
            ],
            down_stones: [
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
            ],
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
        todo!()
    }

    pub fn revoke_move(&mut self, step: &Step) {
        todo!()
    }

    pub fn can_move(&mut self, step: &Step) {
        todo!()
    }
    // private
    fn is_king_meeted(&mut self) -> bool {
        true
    }
    fn can_king_move(&mut self, step: &Step) -> bool {
        todo!()
    }
    fn can_mandarin_move(&mut self, step: &Step) -> bool {
        todo!()
    }
    fn can_bishop_move(&mut self, step: &Step) -> bool {
        todo!()
    }
    fn can_knight_move(&mut self, step: &Step) -> bool {
        todo!()
    }
    fn can_rook_move(&mut self, step: &Step) -> bool {
        todo!()
    }
    fn can_cannon_move(&mut self, step: &Step) -> bool {
        todo!()
    }
    fn can_pawn_move(&mut self, step: &Step) -> bool {
        todo!()
    }

    // 走法生成器
    fn generate_possible_steps(&mut self) -> Vec<Step> {
        todo!()
    }
}

// 輸出棋盤
impl Display for StoneMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "　１２３４５６７８９　")?;
        writeln!(f, "　＿＿＿＿＿＿＿＿＿　")?;

        for i in 0..10 {
            write!(f, "｜")?;
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
            writeln!(f, "｜")?;
        }

        writeln!(f, "　￣￣￣￣￣￣￣￣￣　")?;
        writeln!(f, "　１２３４５６７８９　")?;

        Ok(())
    }
}
