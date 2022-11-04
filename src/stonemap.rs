use std::fmt::Display;

use crate::stone::Stone;
use crate::stone::StoneType;

#[derive(Clone, Copy)]
enum StoneIndex {
    Alive(i32, i32),
    Dead(i32, i32),
}

#[derive(Clone)]
pub struct StoneMap {
    stone_map: [[Option<Stone>; 9]; 10],
    up_stones: [StoneIndex; 16],
    down_stones: [StoneIndex; 16],
}

use StoneIndex::Alive;

impl StoneMap {
    fn new() -> Self {
        return StoneMap {
            stone_map: [
                [None; 9], [None; 9], [None; 9], [None; 9], [None; 9], [None; 9], [None; 9],
                [None; 9], [None; 9], [None; 9],
            ],
            up_stones: [
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
            ],
            down_stones: [
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
                Alive(0, 0),
            ],
        };
    }
}

impl Display for StoneMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
