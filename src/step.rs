use std::clone;

use crate::stone::Stone;

#[derive(Clone, Copy)]
pub struct Step {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub killed: Option<Stone>,
}

impl Step {
    pub fn new(from: (usize, usize), to: (usize, usize), killed: Option<Stone>) -> Self {
        Step { from, to, killed }
    }
}
