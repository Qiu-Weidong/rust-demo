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
}

impl Stone {
    pub fn new(stone_type: StoneType, camp:Camp) -> Self {
        Stone { stone_type, camp }
    }
}
