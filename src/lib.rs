pub mod utils;
pub mod game;
pub mod config;

trait Moveable {
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn move_down(&mut self);
    fn rotate(&mut self);
}