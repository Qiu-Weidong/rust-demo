use crate::stone::Stone;

#[derive(Clone, Copy)]
pub struct Step {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub killed: Option<Stone>,
    pub weight: i32,
}

