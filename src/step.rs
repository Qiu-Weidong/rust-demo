use std::clone;

use crate::stone::Stone;

#[derive(Clone, Copy)]
pub struct Step {
    from: (i32, i32),
    to: (i32, i32),
    killed: Option<Stone>,
}

impl Step {
    pub fn new(from: (i32, i32), to: (i32, i32), killed: Option<Stone>) -> Self {
        Step { from, to, killed }
    }
}
