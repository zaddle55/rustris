use crate::utils::Point;
use crate::utils::Direction;

use rand::seq::IteratorRandom;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

use std::str::FromStr;
use std::time::Duration;

use rand::prelude::*;

use itertools::Itertools;

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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Block {
    shape: Option<Shape>,
    is_empty: bool,
}

#[derive(Debug, Clone)]
pub struct Tetromino {
    pub shape: Shape,
    pub points: Vec<Point>,
    pub leading_point: Point,
}

impl Tetromino {

    pub fn random() -> Self {
        let shape = Shape::iter().choose(&mut thread_rng()).unwrap();
        let points = match shape {
            Shape::I => vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 3, y: 0 },
            ],
            Shape::J => vec![
                Point { x: 0, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 1 },
            ],
            Shape::L => vec![
                Point { x: 2, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 1 },
            ],
            Shape::O => vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
            ],
            Shape::S => vec![
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
            ],
            Shape::T => vec![
                Point { x: 1, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 1 },
            ],
            Shape::Z => vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 1 },
            ],
        };
        let leading_point = Point { x: 0, y: 0 };
        Self {
            shape,
            points,
            leading_point,
        }
    }

    pub fn translate(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                self.points.iter_mut().for_each(|p| p.y -= 1);
                self.leading_point.y -= 1;
            }
            Direction::Down => {
                self.points.iter_mut().for_each(|p| p.y += 1);
                self.leading_point.y += 1;
            }
            Direction::Left => {
                self.points.iter_mut().for_each(|p| p.x -= 1);
                self.leading_point.x -= 1;
            }
            Direction::Right => {
                self.points.iter_mut().for_each(|p| p.x += 1);
                self.leading_point.x += 1;
            }
        }
    }

    pub fn rotate(&mut self) {
        let mut new_points = self.points.clone();
        for p in new_points.iter_mut() {
            let x = p.x;
            p.x = -p.y;
            p.y = x;
        }
        self.points = new_points;
    }

    pub fn consume(&mut self, _move: &Move) {
        for _ in 0.._move.distance() {
            self.translate(_move.direction());
        }
    }
    
}

pub struct Move {
    elapsed_time: Duration,
    distance: i32,
    direction: Direction,
    is_once: bool,
}

impl Move {
    pub fn new(elapsed_time: Duration, distance: i32, direction: Direction, is_once: bool) -> Self {
        Self {
            elapsed_time,
            distance,
            direction,
            is_once,
        }
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn distance(&self) -> i32 {
        self.distance
    }
    
}

pub struct Game {
    height: i32,
    width: i32,
    pub cells: Vec<Vec<Block>>,
    acc: Vec<i32>,
    pub living_tetromino: Tetromino,
    pub next_tetromino: Tetromino,
    moves: Vec<Move>,
}

impl Game {
    const MASKED_HEIGHT: i32= 4;

    fn handle_move(&mut self) {
        match self.moves.first() {
            Some(m) => {
                let mut tetromino = self.living_tetromino.clone();
                tetromino.consume(&m);
                for (_, p) in tetromino.points.iter().enumerate() {
                    if p.x < 0 || p.x >= self.width || p.y >= self.height {
                        return;
                    }
                    if p.y >= 0 {
                        if self.cells[p.y as usize][p.x as usize].is_empty {
                            self.cells[p.y as usize][p.x as usize].shape = Some(tetromino.shape);
                            self.cells[p.y as usize][p.x as usize].is_empty = false;
                        } else {
                            return;
                        }
                    }
                }
                if m.is_once {
                    self.moves.remove(0);
                }
            }
            None => {}         
        }
        
    }
    
    fn is_boundary_collision(&self, tetromino: &Tetromino) -> bool {
        for (_, p) in tetromino.points.iter().enumerate() {
            if p.x < 0 || p.x >= self.width || p.y >= self.height {
                return true;
            }
        }
        false
    }

    fn is_block_collision(&self, tetromino: &Tetromino) -> bool {
        for (_, p) in tetromino.points.iter().enumerate() {
            if p.y >= 0 {
                if !self.cells[p.y as usize][p.x as usize].is_empty {
                    return true;
                }
            }
        }
        false
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_from_str() {
        assert_eq!(Shape::from_str("I").unwrap(), Shape::I);
        assert_eq!(Shape::from_str("J").unwrap(), Shape::J);
        assert_eq!(Shape::from_str("L").unwrap(), Shape::L);
        assert_eq!(Shape::from_str("O").unwrap(), Shape::O);
        assert_eq!(Shape::from_str("S").unwrap(), Shape::S);
        assert_eq!(Shape::from_str("T").unwrap(), Shape::T);
        assert_eq!(Shape::from_str("Z").unwrap(), Shape::Z);
    }

    #[test]
    fn test_tetromino_random() {
        let tetromino = Tetromino::random();
        assert!(Shape::iter().any(|s| s == tetromino.shape));
    }

    #[test]
    fn test_tetromino_translate() {
        let mut tetromino = Tetromino {
            shape: Shape::I,
            points: vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 3, y: 0 },
            ],
            leading_point: Point { x: 0, y: 0 },
        };
        let move1 = Move::new(Duration::from_secs(1), 1, Direction::Down, false);
        let move2 = Move::new(Duration::from_secs(1), 1, Direction::Right, false);
        let move3 = Move::new(Duration::from_secs(1), 1, Direction::Left, false);
        tetromino.consume(&move1);
        assert_eq!(tetromino.points, vec![
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 3, y: 1 },
        ]);
        tetromino.consume(&move2);
        assert_eq!(tetromino.points, vec![
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 3, y: 1 },
            Point { x: 4, y: 1 },
        ]);
        tetromino.consume(&move3);
        assert_eq!(tetromino.points, vec![
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 3, y: 1 },
        ]);
    }

    #[test]
    fn test_tetromino_rotate() {
        let mut tetromino = Tetromino {
            shape: Shape::I,
            points: vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 3, y: 0 },
            ],
            leading_point: Point { x: 0, y: 0 },
        };}
    }