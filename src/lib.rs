mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rust-js-snake-game!");
}

/// holds a position on the map
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// Debug lets it be tested. PartialEq lets it be used in assert_eq
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    UP = 0,
    DOWN = 1,
    RIGHT = 2,
    LEFT = 3,
}

pub struct Board {
    pub board: Vec<Vec<u8>>,
}

#[wasm_bindgen]
pub struct Game {
    pub width: u8,
    pub height: u8,
    pub speed: u8,
    snake: Vec<Position>,
    pub direction: Direction,
    pub food: Position,
    pub score: u8,
    board: Board,
}

#[wasm_bindgen]
impl Game {
    /// the head of the snake will be in index 0 of snake: Vec<Position>
    #[wasm_bindgen(constructor)]
    pub fn new(
        mut width: u8,
        mut height: u8,
        mut speed: u8,
        mut snake_length: u8,
        mut direction: Direction,
    ) -> Game {
        // checking board size
        if width > 100 || width < 8 {
            width = 17
        }
        if height > 100 || height < 8 {
            height = 16
        }

        // checking game speed and initial direction
        if speed > 100 || speed < 1 {
            speed = 10
        }
        direction = Direction::RIGHT;

        // checking snake length
        if snake_length >= width - 3 {
            snake_length = 3
        }

        // creating snake vector
        let snake = vec![Position { x: 0, y: 0 }; snake_length as usize];

        // creating board
        let two_d_vector = vec![vec![0; width as usize]; height as usize];
        let board = Board {
            board: two_d_vector,
        };
        Game {
            width: width,
            height: height,
            speed: speed,
            board: board,
            direction: direction,
            food: Position { x: 5, y: 3 },
            score: 0,
            snake: snake,
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = Game::new(1, 1, 1, 0, Direction::UP);
        assert_eq!(game.direction, Direction::RIGHT);
        assert_eq!(game.height, 16);
        assert_eq!(game.width, 17);
        assert_eq!(game.score, 0);
    }
}
