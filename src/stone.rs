#[derive(Copy, Clone, PartialEq, Debug)]
pub enum StoneType {
    King,
    Mandarin,
    Bishop,
    Knight,
    Rook,
    Cannon,
    Pawn,
}
#[derive(Copy, Clone, PartialEq)]
pub enum Camp {
    Up,
    Down,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Stone {
    pub stone_type: StoneType,
    pub camp: Camp,
    pub id: usize
}

impl Stone {
    pub fn new(stone_type: StoneType, camp:Camp, id: usize) -> Self {
        Stone { stone_type, camp , id }
    }

    pub fn get_char(&self) -> char {
        match self.stone_type {
            StoneType::King => match self.camp {
                Camp::Up => '將',
                Camp::Down => '帥',
            },
            StoneType::Mandarin => match self.camp {
                Camp::Up => '士',
                Camp::Down => '仕',
            },
            StoneType::Bishop => match self.camp {
                Camp::Up => '象',
                Camp::Down => '相',
            },
            StoneType::Knight => match self.camp {
                Camp::Up => '馬',
                Camp::Down => '傌',
            },
            StoneType::Rook => match self.camp {
                Camp::Up => '車',
                Camp::Down => '俥',
            },
            StoneType::Cannon => match self.camp {
                Camp::Up => '砲',
                Camp::Down => '炮',
            },
            StoneType::Pawn => match self.camp {
                Camp::Up => '卒',
                Camp::Down => '兵',
            },
        }
    }
}
/*
编号方法 十六进制
　＿＿＿＿＿＿＿＿＿　
｜０１２３４５６７８｜
｜　　　　　　　　　｜
｜　Ｅ　　　　　Ａ　｜
｜Ｆ　Ｄ　Ｃ　Ｂ　９｜
｜－－－－－－－－－｜
｜－－－－－－－－－｜
｜９　Ｂ　Ｃ　Ｄ　Ｆ｜
｜　Ａ　　　　　Ｅ　｜
｜　　　　　　　　　｜
｜８７６５４３２１０｜
　￣￣￣￣￣￣￣￣￣　
*/