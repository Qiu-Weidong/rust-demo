#[derive(Copy, Clone)]
pub enum StoneType {
    King,
    Mandarin,
    Bishop,
    Knight,
    Rook,
    Cannon,
    Pawn,
}
#[derive(Copy, Clone)]
pub enum Camp {
    Up,
    Down,
}

#[derive(Clone, Copy)]
pub struct Stone {
    stone_type: StoneType,
    camp: Camp,
}

impl Stone {
    fn new(stone_type: StoneType, camp:Camp) -> Self {
        Stone { stone_type, camp }
    }
}
