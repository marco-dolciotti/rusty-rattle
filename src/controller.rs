use std::{collections::VecDeque, sync::Mutex};

use crossterm::event::KeyCode;

use crate::model::{GameState, Model, Orientation};

pub struct Controller {
    model: Model,
    direction_buffer: Mutex<VecDeque<Orientation>>,
}

impl Controller {
    pub fn new(model: Model) -> Self {
        Controller {
            model,
            direction_buffer: Mutex::new(VecDeque::new()),
        }
    }

    pub fn update_model(&mut self) {
        let direction = self.direction_buffer.lock().unwrap().pop_front();
        if self.model.game_state() == GameState::Playing {
            self.model.update(direction);
        }
    }

    pub fn handle_input(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Enter => self.start_game(),
            KeyCode::Char('a') => self.input_direction(Orientation::Left),
            KeyCode::Char('w') => self.input_direction(Orientation::Up),
            KeyCode::Char('d') => self.input_direction(Orientation::Right),
            KeyCode::Char('s') => self.input_direction(Orientation::Down),
            _ => {}
        }
    }

    fn input_direction(&mut self, direction: crate::model::Orientation) {
        match self.model.game_state() {
            GameState::Playing => {
                let mut input_stack = self.direction_buffer.lock().unwrap();
                //only pushes the new direction if the previous one is different
                //(to prevent an issue with repeating keys by holding them down)
                if input_stack
                    .back()
                    .map(|dir| *dir != direction)
                    .unwrap_or(true)
                {
                    input_stack.push_back(direction);
                }
            }
            GameState::TitleScreen | GameState::GameOver => {}
        }
    }

    fn start_game(&mut self) {
        match self.model.game_state() {
            GameState::TitleScreen | GameState::GameOver => self.model.start_game(),
            GameState::Playing => {}
        }
    }
}
