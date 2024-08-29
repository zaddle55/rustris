use crate::utils::Point;
use crate::utils::Direction;

use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, EnumCountMacro, EnumIter)]
pub enum Shape {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "I" => Ok(Shape::I),
            "J" => Ok(Shape::J),
            "L" => Ok(Shape::L),
            "O" => Ok(Shape::O),
            "S" => Ok(Shape::S),
            "T" => Ok(Shape::T),
            "Z" => Ok(Shape::Z),
            _ => Err(()),
        }
    }
}

pub struct Field {
    masked_height: i32,
    cells: Vec<Vec<Block>>,
    acc: Vec<i32>,
}

struct Block {
    shape: Option<Shape>,
    is_empty: bool,
}

pub struct Tetromino {
    shape: Shape,
    points: Vec<Point>,
    leading_point: Point,
}

struct Move {
    elapsed_time: f32,
    distance: i32,
    direction: Direction,
    is_once: bool,
}