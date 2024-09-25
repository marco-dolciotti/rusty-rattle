use std::{collections::VecDeque, sync::Mutex, thread, time::{Duration, Instant}};

use crossterm::event::KeyCode;

use crate::model::{GameState, Model, Orientation};

//the time between one game update and the next
const GAME_UPDATE_INTERVAL: Duration = Duration::from_millis(200);

pub struct Controller {
    model: Model,
    direction_buffer: Mutex<VecDeque<Orientation>>,
}

impl Controller {
    pub fn new(model: Model) -> Self {
        Controller {
            model,
            direction_buffer: Mutex::new(VecDeque::new())
        }
    }

    pub fn update_model(&mut self) {
        let direction = self.direction_buffer.lock().unwrap().pop_front();
        match self.model.game_state() {
            GameState::Playing => {
                self.model.update(direction);
            },
            _ => {}
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
            GameState::Playing => self.direction_buffer.lock().unwrap().push_back(direction),
            GameState::TitleScreen | GameState::GameOver => {}
        }
    }

    fn start_game(&mut self){
        match self.model.game_state() {
            GameState::TitleScreen | GameState::GameOver => self.model.start_game(),
            GameState::Playing => {},
        }
    }

}