
use std::cell::Cell;

use rand::{self, Rng};

use crate::views::View;

pub const GRID_HEIGHT: usize = 10;
pub const GRID_WIDTH: usize = 20;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Orientation {
    Up,
    Right,
    Down,
    Left,
}
impl Orientation {
    fn is_opposite(&self, other: Orientation) -> bool {
        matches! ((self, other),
            (Orientation::Up, Orientation::Down) |
            (Orientation::Right, Orientation::Left) |
            (Orientation::Down, Orientation::Up) |
            (Orientation::Left, Orientation::Right)
        )
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    TitleScreen,
    Playing,
    GameOver,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub enum CellContent {
    #[default]
    Empty,
    Head(Orientation),
    Tail(Orientation),
    Body {
        towards: Orientation,
        from: Orientation,
    },
    Apple,
}

pub struct Model {
    view: Box<dyn View>,
    game_state: GameState,
    snake_len: usize,
    grid: [[CellContent; GRID_WIDTH]; GRID_HEIGHT],
}

impl Model {
    pub fn new(view: Box<dyn View>) -> Self {

        let mut grid = core::array::from_fn(|_| 
                                                core::array::from_fn(|_| 
                                                    CellContent::default()));

        //print title screen
        view.draw_title_screen();

        Model {
            view,
            game_state: GameState::TitleScreen,
            grid,
            snake_len: 4
        }

    }

    //returns the game state after the update
    pub fn update(&mut self, input_direction: Option<Orientation>) -> GameState {

        let new_game_state= GameState::Playing;

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
        let front_coordinates: Option<(usize, usize)> = match input_direction {

            Some(Orientation::Up) => head_y.checked_sub(1).map(|y| (head_x, y)),
            Some(Orientation::Down) => Some((head_x, head_y + 1)),
            Some(Orientation::Left) => head_x.checked_sub(1).map(|x| (x, head_y)),
            Some(Orientation::Right) => Some((head_x + 1, head_y)),

            //if there is no input in the buffer, maintain the previous input_direction
            None => match head {
                CellContent::Head(Orientation::Up) => head_y.checked_sub(1).map(|y| (head_x, y)),
                CellContent::Head(Orientation::Down) => Some((head_x, head_y + 1)),
                CellContent::Head(Orientation::Left) => head_x.checked_sub(1).map(|x| (x, head_y)),
                CellContent::Head(Orientation::Right) => Some((head_x + 1, head_y)),

                _ => panic!("expected head")
            }
        };

        let (front_x, front_y) = match front_coordinates {
            Some(_) => front_coordinates.unwrap(),
            None => {
                self.game_over();
                return GameState::GameOver;
            }
        };

        let front = match self.get_cell(front_x, front_y){
            //wall collision => game over
            None => {
                self.game_over();
                return GameState::GameOver;
            }
            Some(h) => h
        };

        // calculate what to do depending on what is in front of the snake
        match front {
            CellContent::Empty => self.advance_snake(input_direction, false),
            CellContent::Head(_) => panic!("there should not be a head in front of a head"),
            CellContent::Tail(_) => self.advance_snake(input_direction, false),
            CellContent::Body { towards: _, from: _ } => {
                self.game_over();
                return GameState::GameOver;
            },
            CellContent::Apple => {
                self.advance_snake(input_direction, true);
                self.snake_len += 1;
                self.spawn_apple()
            }
        }
        
        //if the game is still going update the view
        self.draw_grid_on_view();


        new_game_state

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
    
    pub fn advance_snake(&mut self, input_direction: Option<Orientation>, apple_is_eaten: bool){
        
        let mut snake_coordinates = self.snake_coordinates().into_iter();

        if !apple_is_eaten {
            let (x_tail, y_tail) = snake_coordinates.next().unwrap();
            let tail = self.get_cell(x_tail, y_tail).unwrap().clone();
            let (o, _, _)= self.next_snake_cell(&tail, x_tail, y_tail);
            let next_tail_orientation = match o.unwrap() {
                CellContent::Body { towards, from: _ } => *towards,
                _ => panic!(),
            };
            self.remove_snake_part((x_tail, y_tail));
            let (x_tail, y_tail) = snake_coordinates.next().unwrap();
            *self.get_cell_mut(x_tail, y_tail).unwrap() = CellContent::Tail(next_tail_orientation);
        }

        let (x_head, y_head) = snake_coordinates.last().expect("snake should have a head");
        let head = self.get_cell_mut(x_head, y_head).expect("snake should have a head");
        match head {
            CellContent::Head(head_orientation) => {
                let walking_direction = input_direction.unwrap_or(*head_orientation);
                //transform the head in body
                *head = CellContent::Body { towards: walking_direction,from: *head_orientation};
                let head = head.clone();
                let (new_head, _, _) = self.next_snake_cell_mut(&head, x_head, y_head);
                let new_head = new_head.expect("head should not point to wall in advance_snake()");
                *new_head = CellContent::Head(walking_direction);
            },
            _ => panic!("last segment should be a head"),
        }

    }

    // returns a vector of tuples, representing the coordinates of the body of the snake, from the tail to the head
    fn snake_coordinates(&self) -> Vec<(usize, usize)>{
        let mut cell_iter;
        let mut cell_opt;
        let mut x_iter; 
        let mut y_iter;

        let mut coordinates = Vec::new();

        //start iteration at the tail
        (x_iter, y_iter) = self.tail_coordinates();
        cell_iter = self.get_cell(x_iter, y_iter).expect("this should be a tail");
        coordinates.push((x_iter, y_iter));

        loop {
            (cell_opt, x_iter, y_iter) = self.next_snake_cell(cell_iter, x_iter, y_iter);
            cell_iter = cell_opt.expect("none of the body segments should point to a wall");
            coordinates.push((x_iter, y_iter));
            if let &CellContent::Head(_) = cell_iter {
                break;
            }
        }

        coordinates

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
            _ => {
                panic!("expected snake cell")
            }
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
                Self::initialize_grid(&mut self.grid);
                self.game_state = GameState::Playing;
            },
            _ => {}
        }
    }

    fn game_over(&mut self) {
        self.game_state = GameState::GameOver;
        self.view.draw_game_over();
    }

    fn draw_grid_on_view(&self) {
        self.view.draw_frame(self.grid.clone());
    }
    
    pub fn game_state(&self) -> GameState {
        self.game_state
    }
    
    fn remove_snake_part(&mut self, (x_tail, y_tail): (usize, usize)) {
        *self.get_cell_mut(x_tail, y_tail).expect("cell should be within grid") = CellContent::Empty;
    }
    fn initialize_grid(grid: &mut [[CellContent; GRID_WIDTH]; GRID_HEIGHT]) {

        //initialize every cell as empty
        grid.iter_mut().flat_map(|row| row.iter_mut()).for_each(|cell|
            *cell = CellContent::Empty
        );

        //initialize snake
        grid[GRID_HEIGHT / 2][GRID_WIDTH / 2] = CellContent::Head(Orientation::Up);
        grid[GRID_HEIGHT / 2 + 1][GRID_WIDTH / 2] = CellContent::Body{ towards: Orientation::Up, from: Orientation::Up};
        grid[GRID_HEIGHT / 2 + 2][GRID_WIDTH / 2] = CellContent::Body{ towards: Orientation::Up, from: Orientation::Up};
        grid[GRID_HEIGHT / 2 + 3][GRID_WIDTH / 2] = CellContent::Tail(Orientation::Up);

        //initialize apple
        grid[GRID_HEIGHT / 2 + 2][GRID_WIDTH / 2 - 1] = CellContent::Apple;
    }
    
}


mod cell_content_iterator;