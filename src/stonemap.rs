use colored::Colorize;
use std::fmt::Display;

use crate::stone::Camp::{self, Down, Up};
use crate::stone::Stone;
use crate::stone::StoneType::{self, Bishop, Cannon, King, Knight, Mandarin, Pawn, Rook};

#[derive(Clone, Copy, PartialEq)]
enum StoneIndex {
    Alive(usize, usize),
    Dead,
}

impl StoneIndex {
    fn get(&self) -> (usize, usize) {
        match self {
            Alive(x, y) => (*x, *y),
            StoneIndex::Dead => panic!("the stone id dead!"),
        }
    }
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

        let mover = mover.unwrap();

        match mover.camp {
            Up => self.up_stones[mover.id] = Alive(step.to.0, step.to.1),
            Down => self.down_stones[mover.id] = Alive(step.to.0, step.to.1),
        }

        if let Some(killed) = step.killed {
            match killed.camp {
                Up => self.up_stones[killed.id] = StoneIndex::Dead,
                Down => self.down_stones[killed.id] = StoneIndex::Dead,
            }
        }
    }

    pub fn revoke_move(&mut self, step: &Step) {
        let mover = self.stone_map[step.to.0][step.to.1];
        assert!(mover != None);
        assert!(self.stone_map[step.from.0][step.from.1] == None);
        self.stone_map[step.from.0][step.from.1] = mover;
        self.stone_map[step.to.0][step.to.1] = step.killed;

        let mover = mover.unwrap();
        match mover.camp {
            Up => self.up_stones[mover.id] = Alive(step.from.0, step.from.1),
            Down => self.down_stones[mover.id] = Alive(step.from.0, step.from.1),
        }

        if let Some(killed) = step.killed {
            match killed.camp {
                Up => self.up_stones[killed.id] = Alive(step.to.0, step.to.1),
                Down => self.down_stones[killed.id] = Alive(step.to.0, step.to.1),
            }
        }
    }

    pub fn can_move(&mut self, step: &Step) -> bool {
        if step.from.1 > 9
            || step.from.1 > 8
            || step.to.0 > 9
            || step.to.1 > 8
            || step.from.0 == step.to.0 && step.from.1 == step.to.1
        {
            return false;
        }
        let mover = self.stone_map[step.to.0][step.to.1];
        if mover == None {
            return false;
        }

        let mover = mover.unwrap();
        if let Some(killed) = step.killed {
            if killed.camp == mover.camp {
                return false;
            }
        }

        let mut ret = match mover.stone_type {
            King => self.can_king_move(step.from, step.to, mover.camp),
            Mandarin => self.can_mandarin_move(step.from, step.to, mover.camp),
            Bishop => self.can_bishop_move(step.from, step.to, mover.camp),
            Knight => self.can_knight_move(step.from, step.to, mover.camp),
            Rook => self.can_rook_move(step.from, step.to, mover.camp),
            Cannon => self.can_cannon_move(step.from, step.to, mover.camp),
            Pawn => self.can_pawn_move(step.from, step.to, mover.camp),
        };
        if !ret {
            return ret;
        }

        self.make_move(step);
        ret = !self.is_king_meeted();
        self.revoke_move(step);
        ret
    }

    // 走法生成器
    pub fn generate_stone_steps(&mut self) -> Vec<Step> {
        todo!()
    }

    pub fn parse_step(&mut self, input: &str) -> Result<Step, String> {
        // 首先转换为 字符数组
        let chars: Vec<char> = input.trim().chars().collect();
        if chars.len() < 4 || chars.len() > 5 {
            // 可能有五个字，如前兵九平八
            return Err(String::from("无法解析输入走步，请输入四字或五字的走步"));
        }

        match chars[0] {
            '将' | '帅' | '將' | '帥' | '王' => self.parse_king_step(&chars),
            '士' | '仕' => self.parse_mandarin_step(&chars),
            '象' | '相' => self.parse_bishop_step(&chars),
            '馬' | '傌' | '马' | '㐷' => self.parse_knight_step(&chars),
            '车' | '伡' | '車' | '俥' => self.parse_rook_step(&chars),
            '炮' | '砲' => self.parse_cannon_step(&chars),
            '兵' | '卒' => self.parse_pawn_step(&chars),
            '前' | '后' | '後' | '二' | '三' | '四' => self.parse_same_line_step(&chars),
            _ => todo!()
        }
    }
    // private
    fn is_king_meeted(&mut self) -> bool {
        // 如果有一个王死了，那么必然不会碰面
        if self.up_stones[4] == StoneIndex::Dead || self.down_stones[4] == StoneIndex::Dead {
            return false;
        }

        // 首先找到两个王
        let up_king = self.up_stones[4].get();
        let down_king = self.down_stones[4].get();

        // 不在一条竖线上
        if up_king.1 != down_king.1 {
            return false;
        }
        let y = up_king.1;
        for x in (up_king.0 + 1)..down_king.0 {
            if let Some(_) = self.stone_map[x][y] {
                return false;
            }
        }

        true
    }
    fn can_king_move(&mut self, from: (usize, usize), to: (usize, usize), camp: Camp) -> bool {
        if to.1 < 3 || to.1 > 5 || camp == Up && to.0 > 2 || camp == Down && to.0 < 7 {
            return false;
        }
        (from.0 - to.0) * (from.0 - to.0) + (from.1 - to.1) * (from.1 - to.1) == 1
    }
    fn can_mandarin_move(&mut self, from: (usize, usize), to: (usize, usize), camp: Camp) -> bool {
        if to.1 < 3 || to.1 > 5 || camp == Up && to.0 > 2 || camp == Down && to.0 < 7 {
            return false;
        }
        (from.0 - to.0) * (from.0 - to.0) + (from.1 - to.1) * (from.1 - to.1) == 2
    }
    fn can_bishop_move(&mut self, from: (usize, usize), to: (usize, usize), camp: Camp) -> bool {
        if camp == Up && to.0 > 4
            || camp == Down && to.0 < 5
            || (from.0 - to.0) * (from.0 - to.0) + (from.1 - to.1) * (from.1 - to.1) != 8
        {
            return false;
        }
        let cx = (from.0 + to.0) >> 1;
        let cy = (from.1 + to.1) >> 1;
        self.stone_map[cx][cy] == None
    }
    fn can_knight_move(&mut self, from: (usize, usize), to: (usize, usize), _camp: Camp) -> bool {
        if (from.0 - to.0) * (from.0 - to.0) + (from.1 - to.1) * (from.1 - to.1) != 5 {
            return false;
        } else if (from.0 - to.0) * (from.0 - to.0) == 1 {
            // 沿着纵向跳了一步，横向跳了两步
            let cx = from.0;
            let cy = (from.1 + to.1) >> 1;
            return self.stone_map[cx][cy] == None;
        } else {
            let cx = (from.0 + to.0) >> 1;
            let cy = from.1;
            return self.stone_map[cx][cy] == None;
        }
    }
    fn can_rook_move(&mut self, from: (usize, usize), to: (usize, usize), _camp: Camp) -> bool {
        if from.0 == to.0 {
            // 横着走
            let miny = if from.1 < to.1 { from.1 } else { to.1 };
            let maxy = if from.1 < to.1 { to.1 } else { from.1 };
            let x = from.0;
            for y in (miny + 1)..(maxy) {
                if let Some(_) = self.stone_map[x][y] {
                    return false;
                }
            }
            return true;
        } else if from.1 == to.1 {
            // 竖着走
            let minx = if from.0 < to.0 { from.0 } else { to.0 };
            let maxx = if from.0 < to.0 { to.0 } else { from.0 };
            let y = from.1;
            for x in (minx + 1)..(maxx) {
                if let Some(_) = self.stone_map[x][y] {
                    return false;
                }
            }
            return true;
        }
        false
    }
    fn can_cannon_move(&mut self, from: (usize, usize), to: (usize, usize), _camp: Camp) -> bool {
        if from.0 == to.0 {
            // 横着走
            let miny = if from.1 < to.1 { from.1 } else { to.1 };
            let maxy = if from.1 < to.1 { to.1 } else { from.1 };
            let x = from.0;
            let mut cnt = 0;
            for y in (miny + 1)..(maxy) {
                if let Some(_) = self.stone_map[x][y] {
                    cnt += 1;
                }
            }
            return if self.stone_map[to.0][to.1] == None {
                cnt == 0
            } else {
                cnt == 1
            };
        } else if from.1 == to.1 {
            // 竖着走
            let minx = if from.0 < to.0 { from.0 } else { to.0 };
            let maxx = if from.0 < to.0 { to.0 } else { from.0 };
            let y = from.1;
            let mut cnt = 0;
            for x in (minx + 1)..(maxx) {
                if let Some(_) = self.stone_map[x][y] {
                    cnt += 1;
                }
            }
            return if self.stone_map[to.0][to.1] == None {
                cnt == 0
            } else {
                cnt == 1
            };
        }
        false
    }
    fn can_pawn_move(&mut self, from: (usize, usize), to: (usize, usize), camp: Camp) -> bool {
        if (from.0 - to.0) * (from.0 - to.0) + (from.1 - to.1) * (from.1 - to.1) != 1 {
            return false;
        }

        match camp {
            Up => {
                if to.0 < from.0 {
                    false
                } else {
                    !(from.0 <= 4 && from.1 != to.1)
                }
            }
            Down => {
                if to.0 > from.0 {
                    false
                } else {
                    !(from.0 >= 5 && from.1 != to.1)
                }
            }
        }
    }

    fn parse_king_step(&mut self, input: &[char]) -> Result<Step, String> {
        todo!()
    }
    fn parse_mandarin_step(&mut self, input: &[char]) -> Result<Step, String> {
        todo!()
    }
    fn parse_bishop_step(&mut self, input: &[char]) -> Result<Step, String> {
        todo!()
    }
    fn parse_knight_step(&mut self, input: &[char]) -> Result<Step, String> {
        todo!()
    }
    fn parse_rook_step(&mut self, input: &[char]) -> Result<Step, String> {
        todo!()
    }
    fn parse_cannon_step(&mut self, input: &[char]) -> Result<Step, String> {
        todo!()
    }
    fn parse_pawn_step(&mut self, input: &[char]) -> Result<Step, String> {
        todo!()
    }
    fn parse_same_line_step(&mut self, input: &[char]) -> Result<Step, String> {
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
                    })?;
                } else {
                    // 输出一个空格
                    let c = if i == 4 || i == 5 { '－' } else { '　' };
                    write!(f, "{}", c)?;
                }
            }
            writeln!(f, "｜")?;
        }

        writeln!(f, "　￣￣￣￣￣￣￣￣￣　")?;
        writeln!(f, "　９８７６５４３２１　")?;

        Ok(())
    }
}
