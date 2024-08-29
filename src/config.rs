use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::{iter, vec};
use std::path::Path;

use strum::{EnumCount, IntoEnumIterator};

use crate::game::Shape;

const CONFIG_PATH: &str = ".conf";

/// Configuration of the game.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Config {
    /// Colors for tetrominoes.
    pub colors: [((u8, u8, u8), Shape); Shape::COUNT],
    /// Key bindings.
    pub keys: [u8; 5],
    /// Game speed.
    pub speed: f32,
    /// Field size.
    pub size: (i32, i32),
}

/// Save configuration to a file(at /.conf).
/// 
/// # Arguments
/// 
/// * `config` - Configuration to save.
/// 
/// # Errors
/// 
/// Returns an error if the file cannot be created or written to.
pub fn save_config(config: Config) -> io::Result<()> {
    let path = Path::new(CONFIG_PATH);
    let mut file = File::create(path)?;

    // Colors
    writeln!(file, "### color configuration ###")?;
    for color in config.colors.iter() {
        writeln!(file, "{:?}: {:?}", color.1, color.0)?;
    }
    writeln!(file)?;

    // Key bindings
    writeln!(file, "### key binding configuration ###")?;
    writeln!(file, "{:?}", config.keys)?;
    writeln!(file)?;

    // Speed
    writeln!(file, "### speed configuration ###")?;
    writeln!(file, "{}", config.speed)?;
    writeln!(file)?;

    // Field size
    writeln!(file, "### field size configuration (height, width) ###")?;
    writeln!(file, "{:?}", config.size)?;

    Ok(())
}

/// Load configuration from a file(at /.conf).
/// 
/// # Errors
/// 
/// Returns an error if the file cannot be read or parsed.
pub fn load_config() -> io::Result<Config> {
    let path = Path::new(CONFIG_PATH);
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut colors = [((0, 0, 0), Shape::I); Shape::COUNT];
    let mut keys = [0; 5];
    let mut speed = 1.0;
    let mut size = (10, 20);

    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        if line.starts_with("### color configuration ###") {
            for color in colors.iter_mut() {
                line.clear();
                reader.read_line(&mut line)?;
                let mut parts = line.trim().split(": ");
                let shape: Shape = parts.next().unwrap().parse().unwrap();
                let mut _parts = parts.next().unwrap().trim_matches(|c| c == '(' || c == ')').split(", ");
                let (r, g, b) = (
                    _parts.next().unwrap().parse().unwrap(),
                    _parts.next().unwrap().parse().unwrap(),
                    _parts.next().unwrap().parse().unwrap(),
                );
                color.0.0 = r;
                color.0.1 = g;
                color.0.2 = b;
                color.1 = shape;
            }
        } else if line.starts_with("### key binding configuration ###") {
            line.clear();
            reader.read_line(&mut line)?;
            keys = parse_array(line.trim()).unwrap().try_into().unwrap();
        } else if line.starts_with("### speed configuration ###") {
            line.clear();
            reader.read_line(&mut line)?;
            speed = line.trim().parse().unwrap();
        } else if line.starts_with("### field size configuration (height, width) ###") {
            line.clear();
            reader.read_line(&mut line)?;
            size = parse_tuple(line.trim()).unwrap();
        }

        line.clear();
        
    }

    Ok(Config {
        colors,
        keys,
        speed,
        size,
    })
}

fn parse_tuple<T: std::str::FromStr>(s: &str) -> Result<(T, T), <T as std::str::FromStr>::Err> {
    let mut iter = s.trim_matches(|c| c == '(' || c == ')').trim().split(", ");
    let a = iter.next().unwrap().parse()?;
    let b = iter.next().unwrap().parse()?;
    Ok((a, b))
}

fn parse_array<T: std::str::FromStr>(s: &str) -> Result<Vec<T>, <T as std::str::FromStr>::Err> {
    let iter = s.trim_matches(|c| c == '[' || c == ']').trim().split(", ");
    let arr: Result<Vec<T>, _> = iter.map(|x| x.parse()).collect();
    arr
}

#[test]
fn test_parse_array() {
    assert_eq!(parse_array::<i32>("[1, 2, 3, 4, 5]").unwrap(), vec![1, 2, 3, 4, 5]);
}