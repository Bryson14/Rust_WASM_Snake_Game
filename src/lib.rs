mod utils;
use rand::{self, Rng};

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

#[derive(Debug, PartialEq, Copy, Clone)]
enum Entity {
    Snake,
    Food,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
struct Snake {
    body: Vec<(u8, u8)>,
}

impl Snake {
    pub fn eat(food_position: (usize, usize)) {}

    /// moves the snake in a direction and returns the new location it moved to
    pub fn move_snake(
        &mut self,
        current_direction: Direction,
        mut new_direction: Direction,
    ) -> Result<((u8, u8), (u8, u8)), &str> {
        // snakes cant make 180 degree turn. If input trys 180 degree turn, then keep going in same direction
        new_direction = match (new_direction, current_direction) {
            (Direction::Up, Direction::Down) => current_direction,
            (Direction::Down, Direction::Up) => current_direction,
            (Direction::Left, Direction::Right) => current_direction,
            (Direction::Right, Direction::Left) => current_direction,
            (_, _) => new_direction,
        };

        let new_pos;
        match new_direction {
            Direction::Up => {
                if self.body[0].1 == 0 {
                    return Err("Hit the top wall");
                }
                new_pos = (self.body[0].0, self.body[0].1 - 1);
            }
            Direction::Down => {
                new_pos = (self.body[0].0, self.body[0].1 + 1);
            }
            Direction::Left => {
                if self.body[0].0 == 0 {
                    return Err("Hit the left wall");
                }
                new_pos = (self.body[0].0 - 1, self.body[0].1);
            }
            Direction::Right => {
                new_pos = (self.body[0].0 + 1, self.body[0].1);
            }
        }

        let old_pos = self.body.pop().unwrap();
        self.body.insert(0, new_pos);
        Ok((new_pos, old_pos))
    }
}

#[derive(Debug)]
struct Board {
    board: Vec<Option<Entity>>,
    width: u8,
    height: u8,
}

impl Board {
    pub fn place_random_food(&mut self) {
        let mut available_spots = Vec::new();
        for (i, option_entity) in self.board.iter().enumerate() {
            match option_entity {
                Some(_) => continue,
                None => available_spots.push(i),
            }
        }

        let choice = available_spots[rand::thread_rng().gen_range(0..available_spots.len())];
        self.board[choice as usize] = Some(Entity::Food);
    }

    pub fn get_index(&self, col: u8, row: u8) -> Result<usize, &'static str> {
        if col > self.width - 1 || row > self.height - 1 {
            Err("Out of bounds")
        } else {
            let idx = (self.width * row + col) as usize;
            Ok(idx)
        }
    }

    pub fn get_entity_at(&self, col: u8, row: u8) -> Result<Option<Entity>, &'static str> {
        let idx = self.get_index(col, row);
        match idx {
            Err(s) => return Err(s),
            Ok(i) => match self.board[i] {
                Some(e) => return Ok(Some(e)),
                None => return Ok(None),
            },
        }
    }
}

#[derive(Debug)]
struct Game {
    board: Board,
    snake_direction: Direction,
    snake: Snake,
}

impl Game {
    pub fn new(mut width: u8, mut height: u8) -> Game {
        if 10 >= width || width > 50 {
            width = 17;
        }
        if 10 >= height || height > 50 {
            height = 15;
        }

        let mut board: Vec<Option<Entity>> = Vec::new();
        let mut snake_body = Vec::new();
        let mut idx = 0;
        for x in 0..width {
            for y in 0..height {
                // place snake half way in the board. Starts out with three
                if height / 2 == y {
                    if width / 2 == x {
                        board[idx + 1] = Some(Entity::Snake);
                        board[idx] = Some(Entity::Snake);
                        board[idx - 1] = Some(Entity::Snake);

                        // adding the positions of the snake
                        snake_body.push(((x + 1) as u8, y as u8));
                        snake_body.push((x as u8, y as u8));
                        snake_body.push(((x - 1) as u8, y as u8));
                    }
                }
                idx += 1;
            }
        }

        let mut myboard = Board {
            board: board,
            height: height,
            width: width,
        };
        myboard.place_random_food();

        Game {
            board: myboard,
            snake_direction: Direction::Right,
            snake: Snake { body: snake_body },
        }
    }

    pub fn tick(&mut self, snake_direction: Option<Direction>) {
        let snake_direction = match snake_direction {
            Some(direction) => direction,
            None => self.snake_direction,
        };

        let r = self.snake.move_snake(self.snake_direction, snake_direction);
        match r {
            Err(e) => {
                println!("Error occured: {}", e);
            }
            Ok(tup) => {
                println!("tuple: {:?}", tup);
                if tup.0 .0 >= self.board.width {
                    println!("Passed the right wall, break");
                } else if tup.0 .1 >= self.board.height {
                    println!("Passed the bottom wall, break");
                } else {
                    let new_pos = self.board.get_index(tup.0 .0, tup.0 .1).unwrap();
                    let old_pos = self.board.get_index(tup.1 .0, tup.1 .1).unwrap();

                    self.board.board[old_pos] = None;
                    self.board.board[new_pos] = Some(Entity::Snake);
                    self.snake_direction = snake_direction;
                }
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_board_new() {
        let w = 10;
        let h = 10;
        let b: Vec<Option<Entity>> = (0..w * h).map(|_| None).collect();
        let board = Board {
            width: w,
            height: h,
            board: b,
        };
    }

    #[test]
    fn test_board_place_random_food() {
        let w = 10;
        let h = 10;
        let b: Vec<Option<Entity>> = (0..w * h).map(|_| None).collect();
        let mut board = Board {
            width: w,
            height: h,
            board: b,
        };
        board.place_random_food();

        let mut found_food = false;
        for pos in board.board.iter() {
            match pos {
                Some(entity) => {
                    if entity == &Entity::Food && !found_food {
                        // checking if place-random-food() placed more than one food
                        found_food = true;
                    }
                }
                None => (),
            }
        }
        assert_eq!(true, found_food);
    }

    #[test]
    fn test_board_get_index() {
        let w = 10;
        let h = 10;
        let b: Vec<Option<Entity>> = (0..w * h).map(|_| None).collect();
        let board = Board {
            width: w,
            height: h,
            board: b,
        };

        assert_eq!(55, board.get_index(5, 5).unwrap());
    }

    #[test]
    fn test_board_get_index2() {
        let w = 10;
        let h = 10;
        let b: Vec<Option<Entity>> = (0..w * h).map(|_| None).collect();
        let board = Board {
            width: w,
            height: h,
            board: b,
        };

        match board.get_index(5, 11) {
            Ok(_) => panic!(),
            Err(s) => assert_eq!(s, "Out of bounds"),
        }
    }

    #[test]
    fn test_board_get_entity_at() {
        let w = 10;
        let h = 10;
        let b: Vec<Option<Entity>> = (0..w * h).map(|_| None).collect();
        let mut board = Board {
            width: w,
            height: h,
            board: b,
        };

        assert_eq!(board.get_entity_at(0, 0), Ok(None));
        board.board[12] = Some(Entity::Snake);
        println!("BOARD\n{:?}\n------", board.board);
        assert_eq!(board.get_entity_at(2, 1), Ok(Some(Entity::Snake)));
        let e = board.get_entity_at(15, 20);
        assert_eq!(e.unwrap_err(), "Out of bounds");
    }

    /// trying to the right in the same direction.
    #[test]
    fn test_snake_move_snake() {
        let mut snake_body = Vec::new();
        // placing snake
        snake_body.push((2 as u8, 0 as u8));
        snake_body.push((1 as u8, 0 as u8));
        snake_body.push((0 as u8, 0 as u8));

        let mut snake = Snake { body: snake_body };
        let current_direction = Direction::Right;
        let (new_pos, old_pos) = snake
            .move_snake(current_direction, Direction::Right)
            .unwrap();
        assert_eq!(new_pos, (3, 0));
        assert_eq!(old_pos, (0, 0));
    }

    /// trying to move 180 degrees. Should continue straight
    #[test]
    fn test_snake_move_snake2() {
        let mut snake_body = Vec::new();
        // placing snake
        snake_body.push((2 as u8, 0 as u8));
        snake_body.push((1 as u8, 0 as u8));
        snake_body.push((0 as u8, 0 as u8));

        let mut snake = Snake { body: snake_body };
        let current_direction = Direction::Right;
        let (new_pos, old_pos) = snake
            .move_snake(current_direction, Direction::Left)
            .unwrap();
        assert_eq!(new_pos, (3, 0));
        assert_eq!(old_pos, (0, 0));
    }

    /// trying to move down
    #[test]
    fn test_snake_move_snake3() {
        let mut snake_body = Vec::new();
        // placing snake
        snake_body.push((2 as u8, 0 as u8));
        snake_body.push((1 as u8, 0 as u8));
        snake_body.push((0 as u8, 0 as u8));

        let mut snake = Snake { body: snake_body };
        let current_direction = Direction::Right;
        let (new_pos, old_pos) = snake
            .move_snake(current_direction, Direction::Down)
            .unwrap();
        assert_eq!(new_pos, (2, 1));
        assert_eq!(old_pos, (0, 0));
    }

    /// trying to move up into wall
    #[test]
    fn test_snake_move_snake4() {
        let mut snake_body = Vec::new();
        // placing snake
        snake_body.push((2 as u8, 0 as u8));
        snake_body.push((1 as u8, 0 as u8));
        snake_body.push((0 as u8, 0 as u8));

        let mut snake = Snake { body: snake_body };
        let current_direction = Direction::Right;
        assert_eq!(
            true,
            snake.move_snake(current_direction, Direction::Up).is_err()
        );
    }

    #[test]
    fn test_game_new() {
        let game = Game::new(10, 10);
        assert_eq!(game.board.height, 10);
        assert_eq!(game.board.width, 10);
    }
}
