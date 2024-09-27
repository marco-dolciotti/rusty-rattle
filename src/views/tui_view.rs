use std::io::Write;

use crate::model::{
    CellContent,
    Orientation, GRID_HEIGHT, GRID_WIDTH,
};

use super::View;

#[derive(Default)]
pub struct TuiView {
}

impl View for TuiView {
    fn draw_title_screen(&self) {
        Self::draw_title_screen();
    }

    fn draw_frame(&self, grid: [[CellContent; GRID_WIDTH]; GRID_HEIGHT]) {
        Self::draw_grid(grid);
    }

    fn draw_game_over(&self) {
        Self::draw_game_over()
    }
}

impl TuiView {

    pub fn new() -> Self {
        TuiView {
        }
    }


    fn draw_grid(grid: [[CellContent; GRID_WIDTH]; GRID_HEIGHT]) {

        // clears the screen using ANSI escape codes
        print!("\x1B[2J\x1B[1;1H");

        // top box wall
        print!("╔");       
        for _ in 0..GRID_WIDTH {
            print!("═");       
        }
        print!("╗");       
        print!("\n\r");

        for row in grid {
            print!("║");
            for cell in row {
                Self::draw_cell(&cell)
            }
            print!("║");
            print!("\n\r");
        }

        // bottom box wall
        print!("╚");
        for _ in 0..GRID_WIDTH {
            print!("═");
        }
        print!("╝");
        print!("\n\r");

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
   

                                                                                                 
                                                                                                 

 
    fn draw_game_over() {

        print!("\n\r");
        print!("\n\r");
        println!(" ██████   █████  ███    ███ ███████      ██████  ██    ██ ███████ ██████  \r");
        println!("██       ██   ██ ████  ████ ██          ██    ██ ██    ██ ██      ██   ██ \r");
        println!("██   ███ ███████ ██ ████ ██ █████       ██    ██ ██    ██ █████   ██████  \r");
        println!("██    ██ ██   ██ ██  ██  ██ ██          ██    ██  ██  ██  ██      ██   ██ \r");
        println!(" ██████  ██   ██ ██      ██ ███████      ██████    ████   ███████ ██   ██ \r");
        print!("\n\r");
        print!("\n\r");
        println!("                  press enter to continue, esc to quit");


    }

    fn draw_title_screen() {
        // clears the screen using ANSI escape codes
        print!("\x1B[2J\x1B[1;1H");

        println!("Welcome to:");
        print!("\n\r");
        print!("\n\r");
        println!("██████  ██    ██ ███████ ████████ ██    ██     ██████   █████  ████████ ████████ ██      ███████ ");
        println!("██   ██ ██    ██ ██         ██     ██  ██      ██   ██ ██   ██    ██       ██    ██      ██      ");
        println!("██████  ██    ██ ███████    ██      ████       ██████  ███████    ██       ██    ██      █████   ");
        println!("██   ██ ██    ██      ██    ██       ██        ██   ██ ██   ██    ██       ██    ██      ██      ");
        println!("██   ██  ██████  ███████    ██       ██        ██   ██ ██   ██    ██       ██    ███████ ███████ ");
        print!("\n\r");
        print!("\n\r");
        println!("                                          controls:");
        println!("                                            wads to move");
        println!("                                            esc to quit");
        println!("                                          press enter to continue");
    }
}




mod tests;
