#[derive(Copy, Clone, PartialEq)]
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