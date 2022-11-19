use colored::Colorize;
use std::fmt::Display;


use crate::stone::Camp::{self, Down, Up};
use crate::stone::Stone;
use crate::stone::StoneType::{Bishop, Cannon, King, Knight, Mandarin, Pawn, Rook};

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
        let mut result = StoneMap {
            stone_map: [[None; 9]; 10],
            up_stones: [StoneIndex::Dead; 16],
            down_stones: [StoneIndex::Dead; 16],
            turn: Down,
        };
        result.start();
        result
    }

    pub fn start(&mut self) {
        self.stone_map = [[None; 9]; 10]; // 先清空

        self.up_stones = [
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
        self.down_stones = [
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
        self.turn = Down;

        let stone_types = [
            Rook, Knight, Bishop, Mandarin, King, Mandarin, Bishop, Knight, Rook, Pawn, Cannon,
            Pawn, Pawn, Pawn, Cannon, Pawn,
        ];

        // 将棋子摆放到棋盘上。
        for i in 0..stone_types.len() {
            if let Alive(x, y) = self.up_stones[i] {
                self.stone_map[x][y] = Some(Stone::new(stone_types[i], Up, i));
            }
        }

        for i in 0..stone_types.len() {
            if let Alive(x, y) = self.down_stones[i] {
                self.stone_map[x][y] = Some(Stone::new(stone_types[i], Down, i));
            }
        }
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
        if mover.camp != self.turn { return false; }
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
        let stones = match self.turn {
            Up => self.up_stones,
            Down => self.down_stones
        };

        let mut result : Vec<Step> = Vec::new();
        for stone_index in stones.iter() {
            if let Alive(x, y) = stone_index {
                // 生成它的走步, 这里我们断言mover一定存在
                let mover = self.stone_map[*x][*y].unwrap();
                match mover.stone_type {
                    King => self.generate_king_steps((*x, *y), &mut result),
                    Mandarin => self.generate_mandarin_steps((*x, *y), &mut result),
                    Bishop => self.generate_bishop_steps((*x, *y), &mut result),
                    Knight => self.generate_knight_steps((*x, *y), &mut result),
                    Rook => self.generate_rook_steps((*x, *y), &mut result),
                    Cannon => self.generate_cannon_steps((*x, *y), &mut result),
                    Pawn => self.generate_pawn_steps((*x, *y), &mut result),
                }
            }
        }
        result
    }
    
    pub fn make_step(&self, from: (usize, usize), to: (usize, usize)) -> Step {
        Step {
            from, to,
            killed: self.stone_map[to.0][to.1], 
        }
    }
    
    pub fn get_current_stones(&self) -> &[StoneIndex] {
        match self.turn {
            Up => &self.up_stones,
            Down => &self.down_stones,
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


    // 走法生成器
    fn generate_king_steps(&mut self, from: (usize, usize), result: &mut Vec<Step>) {
        self.generate_king_or_pawn_steps(from, result)
    }
    fn generate_mandarin_steps(&mut self, from: (usize, usize), result: &mut Vec<Step>) {
        let dx = [ -1,  1,  -1,  1];
        let dy = [ -1, -1,   1,  1];

        for i in 0..4 {
            let x : i32 = from.0 as i32 + dx[i];
            let y : i32 = from.1 as i32 + dy[i];

            if x >= 0 && x <= 9 && y >= 0 && y <= 8 {
                let step = self.make_step(from, (x as usize, y as usize));
                if self.can_move(&step) {
                    result.push(step);
                }
            }
        }
    }
    fn generate_bishop_steps(&mut self, from: (usize, usize), result: &mut Vec<Step>) {
        let dx = [ -2,  2,  -2,  2];
        let dy = [ -2, -2,   2,  2];

        for i in 0..4 {
            let x : i32 = from.0 as i32 + dx[i];
            let y : i32 = from.1 as i32 + dy[i];

            if x >= 0 && x <= 9 && y >= 0 && y <= 8 {
                let step = self.make_step(from, (x as usize, y as usize));
                if self.can_move(&step) {
                    result.push(step);
                }
            }
        }
    }
    fn generate_knight_steps(&mut self, from: (usize, usize), result: &mut Vec<Step>) {
        let dx = [ -1, -1,   1,  1, -2, -2,  2,  2];
        let dy = [ -2,  2,  -2,  2, -1,  1, -1, -1];

        for i in 0..8 {
            let x : i32 = from.0 as i32 + dx[i];
            let y : i32 = from.1 as i32 + dy[i];

            if x >= 0 && x <= 9 && y >= 0 && y <= 8 {
                let step = self.make_step(from, (x as usize, y as usize));
                if self.can_move(&step) {
                    result.push(step);
                }
            }
        }
    }
    fn generate_rook_steps(&mut self, from: (usize, usize), result: &mut Vec<Step>) {
        self.generate_rook_or_cannon_steps(from, result)
    }
    fn generate_cannon_steps(&mut self, from: (usize, usize), result: &mut Vec<Step>) {
        self.generate_rook_or_cannon_steps(from, result)
    }
    fn generate_pawn_steps(&mut self, from: (usize, usize), result: &mut Vec<Step>) {
        self.generate_king_or_pawn_steps(from, result)
    }
    fn generate_king_or_pawn_steps(&mut self, from: (usize, usize), result: &mut Vec<Step>) {
        let dx = [1, -1,  0,  0,];
        let dy = [0,  0,  1, -1,];

        for i in 0..4 {
            let x : i32 = from.0 as i32 + dx[i];
            let y : i32 = from.1 as i32 + dy[i];

            if x >= 0 && x <= 9 && y >= 0 && y <= 8 {
                let step = self.make_step(from, (x as usize, y as usize));
                if self.can_move(&step) {
                    result.push(step);
                }
            }
        }
    }
    fn generate_rook_or_cannon_steps(&mut self, from: (usize, usize), result: &mut Vec<Step>) {
        for x in 0..10 {
            let step = self.make_step(from, (x, from.1));
            if self.can_move(&step) { result.push(step); }
        }
        for y in 0..9 {
            let step = self.make_step(from, (from.0, y));
            if self.can_move(&step) { result.push(step); }
        }
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
                    match stone.camp {
                        Camp::Up => write!(f, "{}", stone.get_char().to_string().bright_black())?,
                        Camp::Down => write!(f, "{}", stone.get_char().to_string().red())?,
                    };
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
