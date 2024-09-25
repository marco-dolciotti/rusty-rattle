use std::{collections::VecDeque, sync::{self, mpsc}};

use rand::{self, Rng};

use crate::views::View;

pub const GRID_HEIGHT: usize = 30;
pub const GRID_WIDTH: usize = 40;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Orientation {
    Up,
    Right,
    Down,
    Left,
}
impl Orientation {
    fn is_opposite(&self, other: Orientation) -> bool {
        match (self, other) {

            (Orientation::Up, Orientation::Down) |
            (Orientation::Right, Orientation::Left) |
            (Orientation::Down, Orientation::Up) |
            (Orientation::Left, Orientation::Right) => true,

            _ => false,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    TitleScreen,
    Playing,
    GameOver,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CellContent {
    Empty,
    Head(Orientation),
    Tail(Orientation),
    Body {
        towards: Orientation,
        from: Orientation,
    },
    Apple,
}
impl CellContent {
    fn is_snake(&self) -> bool {
        match self {
            CellContent::Empty | CellContent::Apple => false,
            CellContent::Head(_) | CellContent::Tail(_) | CellContent::Body { towards: _, from: _ } => true,
        }
    }
    
    fn is_head(&self) -> bool {
        match self {
            CellContent::Head(_) => true,
            _ => false
        }
    }
}

pub struct Model {
    view: Box<dyn View>,
    game_state: GameState,
    snake_len: usize,
    grid: [[CellContent; GRID_WIDTH]; GRID_HEIGHT],
}

impl Model {
    pub fn new(view: Box<dyn View>) -> Self {

        Model {
            view,
            game_state: GameState::TitleScreen,
            grid: [[CellContent::Empty; GRID_WIDTH]; GRID_HEIGHT],
            snake_len: 4
        }
    }
    // return true if the game is over
    pub fn update(&mut self, input_direction: Option<Orientation>) -> bool {

        let mut game_over = false;

        let (head_x, head_y) = self.head_coordinates();

        let head = self.get_cell(head_x, head_y).expect("head should be inside grid");

        //cannot change to the opposite direction instantly
        let mut input_direction = input_direction;
        if let CellContent::Head(prev_direction) = head {
            if let Some(new_direction) = input_direction {
                if new_direction.is_opposite(*prev_direction) {
                    input_direction = None;
                }
            }
        }

        //get the cell in front of the head
        let (front_x, front_y) = match input_direction {

            Some(Orientation::Up) => (head_x, head_y - 1),
            Some(Orientation::Down) => (head_x, head_y + 1),
            Some(Orientation::Left) => (head_x - 1, head_y),
            Some(Orientation::Right) => (head_x + 1, head_y),

            //if there is no input in the buffer, maintain the previous input_direction
            None => match head {
                CellContent::Head(Orientation::Up) => (head_x, head_y - 1),
                CellContent::Head(Orientation::Down) => (head_x, head_y + 1),
                CellContent::Head(Orientation::Left) => (head_x - 1, head_y),
                CellContent::Head(Orientation::Right) => (head_x + 1, head_y),

                _ => panic!("expected head")
            }
        };

        let front = match self.get_cell(front_x, front_y){
            //wall collision => game over
            None => return true,
            Some(h) => h
        };

        // calculate what to do depending on what is in front of the snake
        match front {
            CellContent::Empty => self.advance_snake(input_direction, false),
            CellContent::Head(_) => panic!("there should not be a head in front of a head"),
            CellContent::Tail(_) => self.advance_snake(input_direction, false),
            CellContent::Body { towards: _, from: _ } => {
                game_over = true;
            },
            CellContent::Apple => {
                self.advance_snake(input_direction, true);
                self.spawn_apple()
            }
        }
        
        match game_over {
            true => {
                self.view.draw_game_over();
                self.game_state = GameState::GameOver;
            }
            false => {}
        };

        return game_over;

    }
    
    fn head_coordinates(&self) -> (usize, usize) {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let CellContent::Head(_) = cell {
                    return (x, y)
                }
            }
        }
        panic!("no head found")
    }

    fn tail_coordinates(&self) -> (usize, usize) {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let CellContent::Tail(_) = cell {
                    return (x, y)
                }
            }
        }
        panic!("no tail found")
    }
    
    fn get_cell(&self, x: usize, y: usize) -> Option<&CellContent> {
        self.grid.get(y)?.get(x)
    }

    fn get_cell_mut(&mut self, x: usize, y: usize) -> Option<&mut CellContent> {
        self.grid.get_mut(y)?.get_mut(x)
    }
    
    fn advance_snake(&mut self, input_direction: Option<Orientation>, apple_is_eaten: bool) {

        let (mut x_iter, mut y_iter) = self.tail_coordinates();


        // start iteration at the tail
        let mut cell_iter = self.get_cell_mut(x_iter, y_iter).expect("tail should not be outside the grid");
        let cell_opt ;

        //eliminate the tail if the apple was not eaten
        if !apple_is_eaten {

            //delete tail
            *cell_iter = CellContent::Empty;

            let cell_cloned = cell_iter.clone();
            (cell_opt, x_iter, y_iter) = self.next_snake_cell_mut(&cell_cloned, x_iter, y_iter);


            //iterate to the cell after the tail and transform it to a tail
            cell_iter = cell_opt.expect("tail should not point to wall");
            match cell_iter {
                CellContent::Body { towards, from: _} => *cell_iter = CellContent::Tail(*towards),
                _ => panic!("tail should point to body")
            }

        }

        //now the cell_iter should point to the tail of the snake

        let cell_iter = &cell_iter.clone();

        //iterate on all the body and exit when encountering the head
        while cell_iter.is_snake() && !cell_iter.is_head() {

            (_, x_iter, y_iter) = self.next_snake_cell(&cell_iter, x_iter, y_iter);

        }

        //now cell_iter should point to the head

        let mut cell_iter = self.get_cell_mut(x_iter, y_iter).unwrap();
        let cell_opt ;

        if let CellContent::Head(prev_direction) = cell_iter {

            //the direction the snake moves towards this frame
            let walking_direction = input_direction.unwrap_or(*prev_direction);

            *cell_iter = CellContent::Body { towards: walking_direction, from: *prev_direction};

            //iterate to the next cell and transform it into a head
            let cell_cloned = cell_iter.clone();
            (cell_opt, _, _) = self.next_snake_cell_mut(&cell_cloned, x_iter, y_iter);

            cell_iter = cell_opt.expect("head segment should not point at wall");

            *cell_iter = CellContent::Head(walking_direction);

        }
        else {
            panic!("body should point to head")
        }

    }

    fn next_snake_cell_mut(&mut self, cell_iter: &CellContent, tail_x: usize, tail_y: usize) -> (Option<&mut CellContent>, usize, usize) {
        let (next_x, next_y) = match cell_iter {
            CellContent::Body { towards: orientation, from: _ } |
            CellContent::Head(orientation) |
            CellContent::Tail(orientation) => match orientation {
                Orientation::Up => (tail_x, tail_y - 1),
                Orientation::Right => (tail_x + 1, tail_y),
                Orientation::Down => (tail_x, tail_y + 1),
                Orientation::Left => (tail_x - 1, tail_y),
            },
            _ => panic!("expected snake cell")
        };

        let next_cell = self.get_cell_mut(next_x, next_y);

        (next_cell, next_x, next_y)
    }

    fn next_snake_cell(&self, cell_iter: &CellContent, tail_x: usize, tail_y: usize) -> (Option<&CellContent>, usize, usize) {
        let (next_x, next_y) = match cell_iter {
            CellContent::Body { towards: orientation, from: _ } |
            CellContent::Head(orientation) |
            CellContent::Tail(orientation) => match orientation {
                Orientation::Up => (tail_x, tail_y - 1),
                Orientation::Right => (tail_x + 1, tail_y),
                Orientation::Down => (tail_x, tail_y + 1),
                Orientation::Left => (tail_x - 1, tail_y),
            },
            _ => panic!("expected snake cell")
        };

        let next_cell = self.get_cell(next_x, next_y);

        (next_cell, next_x, next_y)
    }
    
    fn spawn_apple(&mut self) {
        let available_cells = GRID_WIDTH * GRID_HEIGHT - self.snake_len;
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..available_cells);

        //grab the indexth empty cell and put an apple there
        *self.grid.iter_mut()
            .flat_map(|col| col.iter_mut())
            .filter(|cell| **cell == CellContent::Empty)
            .nth(index)
            .expect("error indexing available cell for the apple")
                = CellContent::Apple;
    }
    
    pub fn start_game(&mut self) {
        match self.game_state {
            GameState::TitleScreen | GameState::GameOver => {
                self.game_state = GameState::Playing;
            },
            _ => {}
        }
    }
    
    pub fn game_state(&self) -> GameState {
        self.game_state
    }
    
}


mod cell_content_iterator;