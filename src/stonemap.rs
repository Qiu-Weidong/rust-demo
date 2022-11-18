use colored::Colorize;
use std::fmt::Display;

use crate::stone::Camp::{self, Down, Up};
use crate::stone::Stone;
use crate::stone::StoneType::{self, Bishop, Cannon, King, Knight, Mandarin, Pawn, Rook};

#[derive(Clone, Copy, PartialEq)]
pub enum StoneIndex {
    Alive(usize, usize),
    Dead,
}

#[derive(Clone)]
pub struct StoneMap {
    pub stone_map: [[Option<Stone>; 9]; 10],
    pub up_stones: [StoneIndex; 16],
    pub down_stones: [StoneIndex; 16],
    pub turn: Camp,
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

    #[allow(dead_code)]
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
        if step.from.0 > 9
            || step.from.1 > 8
            || step.to.0 > 9
            || step.to.1 > 8
            || step.from.0 == step.to.0 && step.from.1 == step.to.1
        {
            return false;
        }
        let mover = self.stone_map[step.from.0][step.from.1];
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

    #[allow(dead_code)] // 走法生成器
    pub fn generate_stone_steps(&mut self) -> Vec<Step> {
        todo!()
    }

    pub fn parse_step(&mut self, input: &str) -> Result<Step, String> {
        // 首先转换为 字符数组
        let chars: Vec<char> = input.trim().chars().collect();

        // 分为两步，首先获取棋子位置，先不考虑兵的情况
        let (x, y) = self.get_location(&chars)?;

        match chars[0] {
            '兵' | '卒' 
            | '车' | '伡' | '車' | '俥'
            | '炮' | '砲' 
            | '将' | '帅' | '將' | '帥' | '王'  => self.parse_straight_step(&chars, (x, y)),
            _ => Err(format!("未知字符`{}`", chars[0]))
        }
    }
    // private
    fn is_king_meeted(&mut self) -> bool {
        // 如果有一个王死了，那么必然不会碰面
        if let (Alive(x1, y1), Alive(x2, y2)) = (self.up_stones[4], self.down_stones[4]) {
            // 首先找到两个王
            let up_king = (x1, y1);
            let down_king = (x2, y2);

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
            return true;
        } else {
            return false;
        }
    }
    fn can_king_move(&mut self, from: (usize, usize), to: (usize, usize), camp: Camp) -> bool {
        if to.1 < 3 || to.1 > 5 || camp == Up && to.0 > 2 || camp == Down && to.0 < 7 {
            return false;
        }
        (from.0 as i32 - to.0 as i32) * (from.0 as i32 - to.0 as i32)
            + (from.1 as i32 - to.1 as i32) * (from.1 as i32 - to.1 as i32)
            == 1
    }
    fn can_mandarin_move(&mut self, from: (usize, usize), to: (usize, usize), camp: Camp) -> bool {
        if to.1 < 3 || to.1 > 5 || camp == Up && to.0 > 2 || camp == Down && to.0 < 7 {
            return false;
        }
        (from.0 as i32 - to.0 as i32) * (from.0 as i32 - to.0 as i32)
            + (from.1 as i32 - to.1 as i32) * (from.1 as i32 - to.1 as i32)
            == 2
    }
    fn can_bishop_move(&mut self, from: (usize, usize), to: (usize, usize), camp: Camp) -> bool {
        if camp == Up && to.0 > 4
            || camp == Down && to.0 < 5
            || (from.0 as i32 - to.0 as i32) * (from.0 as i32 - to.0 as i32)
                + (from.1 as i32 - to.1 as i32) * (from.1 as i32 - to.1 as i32)
                != 8
        {
            return false;
        }
        let cx = (from.0 + to.0) >> 1;
        let cy = (from.1 + to.1) >> 1;
        self.stone_map[cx][cy] == None
    }
    fn can_knight_move(&mut self, from: (usize, usize), to: (usize, usize), _camp: Camp) -> bool {
        if (from.0 as i32 - to.0 as i32) * (from.0 as i32 - to.0 as i32)
            + (from.1 as i32 - to.1 as i32) * (from.1 as i32 - to.1 as i32)
            != 5
        {
            return false;
        } else if (from.0 as i32 - to.0 as i32) * (from.0 as i32 - to.0 as i32) == 1 {
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
        if (from.0 as i32 - to.0 as i32) * (from.0 as i32 - to.0 as i32)
            + (from.1 as i32 - to.1 as i32) * (from.1 as i32 - to.1 as i32)
            != 1
        {
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

    fn get_location(&self, input: &[char]) -> Result<(usize, usize), String> {
        if input.len() < 4 {
            return Err(String::from("输入字符串过短"));
        }

        match input[0] {
            '将' | '帅' | '將' | '帥' | '王' => self.get_king_location(input),
            '车' | '伡' | '車' | '俥' => self.get_rook_location(input),
            '炮' | '砲' => self.get_cannon_location(input),
            '兵' | '卒' => self.get_pawn_location(input),
            '士' | '仕' => self.get_mandarin_location(input),
            '象' | '相' => self.get_bishop_location(input),
            '馬' | '傌' | '马' | '㐷' => self.get_knight_location(input),
            '前' | '后' | '後' | '中' | '二' | '三' | '四' => match input[1] {
                '车' | '伡' | '車' | '俥' => self.get_rook_location(input),
                '炮' | '砲' => self.get_cannon_location(input),
                '兵' | '卒' => self.get_pawn_location(input),
                '士' | '仕' => self.get_mandarin_location(input),
                '象' | '相' => self.get_bishop_location(input),
                '馬' | '傌' | '马' | '㐷' => self.get_knight_location(input),
                '一' | '壹' | '1' | '１' | '二' | '贰' | '2' | '２' | '三' | '叁' | '3' | '３'
                | '四' | '肆' | '4' | '４' | '五' | '伍' | '5' | '５' | '六' | '陆' | '6'
                | '６' | '七' | '柒' | '7' | '７' | '八' | '捌' | '8' | '８' | '九' | '玖'
                | '9' | '９' => self.get_pawn_location(input),
                _ => Err(format!("未知棋子 `{}`", input[1])),
            },
            _ => Err(format!("未知棋子 `{}`", input[0])),
        }
    }
    fn get_king_location(&self, input: &[char]) -> Result<(usize, usize), String> {
        let col = StoneMap::char_to_number(input[1])?;
        let stones = match self.turn {
            Up => &self.up_stones,
            Down => &self.down_stones,
        };

        // 检查是否活着
        if let Alive(x, y) = stones[4] {
            if y != col {
                Err(format!("将或帅不位于列{}", input[1]))
            } else {
                Ok((x, y))
            }
        } else {
            Err(String::from("将或帅已经死亡."))
        }
    }
    fn get_mandarin_location(&self, input: &[char]) -> Result<(usize, usize), String> {
        self.get_two_location(input, 3, 5)
    }
    fn get_bishop_location(&self, input: &[char]) -> Result<(usize, usize), String> {
        self.get_two_location(input, 2, 6)
    }
    fn get_knight_location(&self, input: &[char]) -> Result<(usize, usize), String> {
        self.get_two_location(input, 1, 7)
    }
    fn get_rook_location(&self, input: &[char]) -> Result<(usize, usize), String> {
        self.get_two_location(input, 0, 8)
    }
    fn get_cannon_location(&self, input: &[char]) -> Result<(usize, usize), String> {
        self.get_two_location(input, 10, 14)
    }
    fn get_pawn_location(&self, input: &[char]) -> Result<(usize, usize), String> {
        let stones = match self.turn {
            Up => &self.up_stones,
            Down => &self.down_stones,
        };

        todo!()
    }
    fn parse_straight_step(&mut self, input: &[char], from: (usize, usize)) -> Result<Step, String> {
        let dest = StoneMap::char_to_number(input[3])?;
        let line = match self.turn { Up => dest - 1, Down => 9 - dest };

        let step = match input[2] {
            '進' | '进' => match self.turn { 
                Up => self.make_step(from, (from.0 + dest, from.1)),
                Down => self.make_step(from, (from.0 - dest, from.1))
            },
            '退' => match self.turn { 
                Up => self.make_step(from, (from.0 - dest, from.1)),
                Down => self.make_step(from, (from.0 + dest, from.1))
            },
            '平' => self.make_step(from, (from.0, line)),
            _ => return  Err(format!("未知操作`{}`", input[2]))
        };

        if self.can_move(&step) { Ok(step) } else { Err(String::from("非法走步.")) } 
    }
    // 一个辅助函数
    fn char_to_number(c: char) -> Result<usize, String> {
        match c {
            '一' | '壹' | '1' | '１' => Ok(1),
            '二' | '贰' | '2' | '２' => Ok(2),
            '三' | '叁' | '3' | '３' => Ok(3),
            '四' | '肆' | '4' | '４' => Ok(4),
            '五' | '伍' | '5' | '５' => Ok(5),
            '六' | '陆' | '6' | '６' => Ok(6),
            '七' | '柒' | '7' | '７' => Ok(7),
            '八' | '捌' | '8' | '８' => Ok(8),
            '九' | '玖' | '9' | '９' => Ok(9),
            _ => Err(format!("无法将字符 `{}` 解析为数字.", c)),
        }
    }
    fn make_step(&self, from: (usize, usize), to: (usize, usize)) -> Step {
        Step {
            from,
            to,
            killed: self.stone_map[to.0][to.1],
        }
    }
    
    // 獲取 士、相、馬、車、炮 的位置
    fn get_two_location(&self, input: &[char], first: usize, second: usize) -> Result<(usize, usize), String> {
        let stones = match self.turn {
            Up => &self.up_stones,
            Down => &self.down_stones,
        };

        let stone_1 = stones[first];
        let stone_2 = stones[second];

        let result: Result<(usize, usize), String> = match input[0] {
            '前' => {
                if let (Alive(x1, y1), Alive(x2, y2))= (stone_1, stone_2) {
                    if y1 != y2 {
                        Err(format!("兩個`{}`不在同一列.", input[1]))
                    }
                    else if self.turn == Up && x1 < x2 || self.turn == Down &&x1 > x2 {
                        Ok((x2, y2))
                    }
                    else { Ok((x1, y1)) }
                }
                else { Err(format!("兩個`{}`不在同一列.", input[1])) }
            }
            '后' | '後' => {
                if let (Alive(x1, y1), Alive(x2, y2))= (stone_1, stone_2) {
                    if y1 != y2 {
                        Err(format!("兩個`{}`不在同一列.", input[1]))
                    }
                    else if self.turn == Up && x1 < x2 || self.turn == Down &&x1 > x2 {
                        Ok((x1, y1))
                    }
                    else { Ok((x2, y2)) }
                }
                else { Err(format!("兩個`{}`不在同一列.", input[1])) }
            }
            _ => {
                let col = StoneMap::char_to_number(input[1])?;
                // col要作處理
                let col = match self.turn { Up => col - 1, Down => 9 - col };

                if let (Alive(x1, y1), Alive(x2, y2))= (stone_1, stone_2) {
                    // 都活著，保證不共綫
                    if y1 == y2 { Err(format!("兩個`{}`共綫, 請使用`前`、`后/後`指明.", input[0])) }
                    else if y1 == col { Ok((x1, y1)) }
                    else if y2 == col { Ok((x2, y2)) }
                    else { Err(format!("兩個`{}`都不在列{}上.", input[0], input[1])) }
                }
                else if let Alive(x, y) = stone_1 { 
                    if y != col { Err(format!("沒有`{}`位於列{}", input[0], input[1])) }
                    else { Ok((x, y)) }
                }
                else if let Alive(x, y) = stone_2 { 
                    if y != col { Err(format!("沒有`{}`位於列{}", input[0], input[1])) }
                    else { Ok((x, y)) }
                }
                else { Err(format!("兩個`{}`都已經陣亡了.", input[0])) }
            }

        };
        result
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

        writeln!(
            f,
            "轮到{}",
            match self.turn {
                Up => "黑方".bright_white(),
                Down => "红方".red(),
            }
        )?;
        Ok(())
    }
}
