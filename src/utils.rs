use rand::Rng;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[must_use]
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    #[must_use]
    pub fn equals(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Point { x, y }
    }
    
}

impl From<Point> for (i32, i32) {
    fn from(point: Point) -> Self {
        (point.x, point.y)
    }
    
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Direction {
    #[default]
    Left,
    Right,
    Up,
    Down,
    
}

impl Direction {
    #[must_use]
    pub fn rand_dir() -> Self {
        match rand::thread_rng().gen_range(0..4) {
            0 => Direction::Left,
            1 => Direction::Right,
            2 => Direction::Up,
            _ => Direction::Down,
        }
    }

    #[must_use]
    pub fn value(&self) -> (i32, i32) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }
}