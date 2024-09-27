use std::{sync::mpsc::{Receiver, Sender}, thread, time::{Duration, Instant}};

use crossterm::{event::{self, KeyCode, KeyEventKind}, terminal};
use crossterm::event::Event as CTEvent;

use crate::controller::Controller;
const GAME_UPDATE_INTERVAL: std::time::Duration = Duration::from_millis(200);

pub enum Event {
    Input(KeyCode),
    Quit,
    Tick,
}

pub fn event_loop(receiver: Receiver<Event>, mut controller: Controller) {
    for event in receiver {
        match event {
            Event::Quit => break,
            Event::Tick => controller.update_model(),
            Event::Input(key_code) => controller.handle_input(key_code),
        }
    }
}

pub fn input_loop(sender: Sender<Event>) {
    terminal::enable_raw_mode().unwrap();
    loop {
        if event::poll(std::time::Duration::from_millis(500)).unwrap() {
            let result = match event::read().unwrap() {
                CTEvent::Key(key_event) => 
                    if key_event.kind == KeyEventKind::Press {
                        match key_event.code {
                            // Exit on Esc key press
                            KeyCode::Esc => {
                                sender.send(Event::Quit).unwrap();
                                break;
                            }, 
                            KeyCode::Char(c @ ('w' | 'a' | 's' | 'd'))=> sender.send(Event::Input(KeyCode::Char(c))),
                            KeyCode::Enter => sender.send(Event::Input(KeyCode::Enter)),
                            _ => Ok(()),
                        }
                    }
                    else { Ok (()) }
                
                _ => Ok(()),
            };
            match result {
                Ok(_) => {},
                Err(_) => break,
            }
        }
    }
    terminal::disable_raw_mode().unwrap();
}

pub fn game_tick_loop(sender: Sender<Event>) {
    loop {
        let start = Instant::now();

        if let Err(_) = sender.send(Event::Tick) {
            break;
        }

        //calculate elapsed time
        let time_elapsed = start.elapsed();
        if time_elapsed < GAME_UPDATE_INTERVAL {
            thread::sleep(GAME_UPDATE_INTERVAL - time_elapsed);
        }
    }
}