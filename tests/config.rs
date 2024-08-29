use rustris::config::Config;
use rustris::config::save_config;

use rustris::game::Shape;

const TEST_CONFIG: Config = Config {
    colors: [
        ((255, 0, 0), Shape::I),
        ((0, 255, 0), Shape::J),
        ((0, 0, 255), Shape::L),
        ((255, 255, 0), Shape::O),
        ((255, 0, 255), Shape::S),
        ((0, 255, 255), Shape::T),
        ((128, 128, 128), Shape::Z),
    ],
    keys: [1, 2, 3, 4, 5],
    speed: 1.0,
    size: (10, 20),
};

#[test]
fn test_save_config() {
    assert!(save_config(TEST_CONFIG).is_ok());
}

#[test]
fn test_load_config() {
    let config = rustris::config::load_config().unwrap();
    assert_eq!(config, TEST_CONFIG);
}

