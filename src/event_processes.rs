use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
    time::Instant,
};

use crossterm::event::Event as CTEvent;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal,
};

use crate::{controller::Controller, Config};

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
                // Exit on Esc key press
                CTEvent::Key(event::KeyEvent {
                    modifiers: _,
                    state: _,
                    kind: KeyEventKind::Press,
                    code: KeyCode::Esc,
                }) => {
                    sender.send(Event::Quit).unwrap();
                    break;
                }
                CTEvent::Key(event::KeyEvent {
                    modifiers: _,
                    state: _,
                    kind: KeyEventKind::Press,
                    code: KeyCode::Char(c @ ('w' | 'a' | 's' | 'd')),
                }) => sender.send(Event::Input(KeyCode::Char(c))),
                CTEvent::Key(event::KeyEvent {
                    modifiers: _,
                    state: _,
                    kind: KeyEventKind::Press,
                    code: KeyCode::Enter,
                }) => sender.send(Event::Input(KeyCode::Enter)),
                _ => Ok(()),
            };
            match result {
                Ok(_) => {}
                Err(_) => break,
            }
        }
    }
    terminal::disable_raw_mode().unwrap();
}

pub fn game_tick_loop(sender: Sender<Event>, config: Config) {
    loop {
        let start = Instant::now();

        if sender.send(Event::Tick).is_err() {
            break;
        }

        //calculate elapsed time
        let time_elapsed = start.elapsed();
        if time_elapsed < config.update_interval {
            thread::sleep(config.update_interval - time_elapsed);
        }
    }
}
