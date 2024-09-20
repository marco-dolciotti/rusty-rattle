use std::{io::{self, Read, Write}, thread, sync::mpsc};

use crossterm::{event::{self, Event, KeyCode}, terminal};

use crate::{controller::Controller, model::{
    CellContent,
    ModelToViewMessage::{self, *},
    Orientation, GRID_HEIGHT, GRID_WIDTH,
}};

use super::View;
pub struct TuiView {
    controller: Controller,
}

impl TuiView {

    pub fn new(controller: Controller, receiver: mpsc::Receiver<ModelToViewMessage>) -> Self {
        TuiView {
            controller
        }
    }

    fn render_loop(receiver: mpsc::Receiver<ModelToViewMessage>) {
        for message in receiver {
            match message {
                Quit => break,
                UpdateFrame(grid) => Self::draw_grid(&grid),
            }
        }
    }

    fn input_loop(&self) {
        terminal::enable_raw_mode().unwrap();
        loop {
            if event::poll(std::time::Duration::from_millis(500)).unwrap() {
                match event::read().unwrap() {
                    Event::Key(key_event) => match key_event.code {
                        KeyCode::Char('q') => break, // Exit on 'q' key press
                        KeyCode::Char(c) => println!("Pressed: {}", c),
                        //TODO
                        KeyCode::Esc => break, // Exit on Esc key press
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        terminal::disable_raw_mode().unwrap();
    }

    fn draw_grid(grid: &[[CellContent; GRID_WIDTH]; GRID_HEIGHT]) {

        // clears the screen using ANSI escape codes
        print!("\x1B[2J\x1B[1;1H");

        // top box wall
        print!("╔");       
        for _ in 0..GRID_WIDTH {
            print!("═");       
        }
        print!("╗");       
        print!("\n");

        for row in grid {
            print!("║");
            for cell in row {
                Self::draw_cell(&cell)
            }
            print!("║");
            print!("\n");
        }

        // bottom box wall
        print!("╚");
        for _ in 0..GRID_WIDTH {
            print!("═");
        }
        print!("╝");
        print!("\n");

        //flush the output buffer
        std::io::stdout().flush().unwrap();

    }

    fn draw_cell(cell: &CellContent) {
        match cell {
            CellContent::Empty => print!(" "),

            CellContent::Head(orientation) => match orientation {
                Orientation::Up => print!("▲"),
                Orientation::Right => print!("▶"),
                Orientation::Down => print!("▼"),
                Orientation::Left => print!("◀"),
            },

            CellContent::Tail(orientation) => match orientation {
                Orientation::Up => print!("╵"),
                Orientation::Right => print!("╶"),
                Orientation::Down => print!("╷"),
                Orientation::Left => print!("╴"),
            },

            CellContent::Body { towards, from } => match (towards, from) {
                (Orientation::Up, Orientation::Down) => {
                    panic!("impossible snake orientation {:?}", (from, towards))
                }
                (Orientation::Up, Orientation::Left) => print!("┗"),
                (Orientation::Up, Orientation::Up) => print!("┃"),
                (Orientation::Up, Orientation::Right) => print!("┛"),
                (Orientation::Right, Orientation::Down) => print!("┗"),
                (Orientation::Right, Orientation::Left) => {
                    panic!("impossible snake orientation {:?}", (from, towards))
                }
                (Orientation::Right, Orientation::Up) => print!("┏"),
                (Orientation::Right, Orientation::Right) => print!("━"),
                (Orientation::Down, Orientation::Down) => print!("┃"),
                (Orientation::Down, Orientation::Left) => print!("┏"),
                (Orientation::Down, Orientation::Up) => {
                    panic!("impossible snake orientation {:?}", (from, towards))
                }
                (Orientation::Down, Orientation::Right) => print!("┓"),
                (Orientation::Left, Orientation::Down) => print!("┛"),
                (Orientation::Left, Orientation::Left) => print!("━"),
                (Orientation::Left, Orientation::Up) => print!("┓"),
                (Orientation::Left, Orientation::Right) => {
                    panic!("impossible snake orientation {:?}", (from, towards))
                }
            },
            CellContent::Apple => print!("●"),
        }
    }
}

impl View for TuiView {

    fn run(&self, receiver: mpsc::Receiver<ModelToViewMessage>) {
        //read rendering message loop
        thread::spawn(move || Self::render_loop(receiver));

        //user input loop
        thread::spawn(|| self.input_loop());
    }
    
    fn set_controller(&mut self, controller: crate::controller::Controller) {
        self.controller = controller;
    }
}



mod tests;
