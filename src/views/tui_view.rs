use std::io::{self, Write};

use crate::model::{CellContent, Orientation};

use super::View;

// linux colors
#[cfg(any(target_os = "linux", target_os = "macos"))]
const WHITE: &str = "\u{1b}[0m";
#[cfg(any(target_os = "linux", target_os = "macos"))]
const GREEN: &str = "\u{1b}[32m";
#[cfg(any(target_os = "linux", target_os = "macos"))]
const RED: &str = "\u{1b}[31m";
#[cfg(any(target_os = "linux", target_os = "macos"))]
const BLUE: &str = "\u{1b}[34m";

// windows colors
#[cfg(target_os = "windows")]
const WHITE: &str = "[0m";
#[cfg(target_os = "windows")]
const GREEN: &str = "[32m";
#[cfg(target_os = "windows")]
const RED: &str = "[31m";
#[cfg(target_os = "windows")]
const BLUE: &str = "[34m";

// default colors
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
const WHITE: &str = "";
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
const GREEN: &str = "";
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
const RED: &str = "";
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
const BLUE: &str = "";

#[derive(Default)]
pub struct TuiView {}

impl View for TuiView {
    fn draw_title_screen(&self) {
        Self::draw_title_screen();
    }

    fn draw_frame(&self, grid: Vec<Vec<CellContent>>) {
        Self::draw_grid(grid);
    }

    fn draw_game_over(&self, score: usize) {
        Self::draw_game_over(score)
    }
}

impl TuiView {
    pub fn new() -> Self {
        crossterm::style::Colored::set_ansi_color_disabled(false);
        TuiView {}
    }

    fn draw_grid(grid: Vec<Vec<CellContent>>) {
        // clears the screen using ANSI escape codes
        print!("\x1B[2J\x1B[1;1H");

        // top box wall
        Self::print_blue("╔═");
        for _ in 0..grid[0].len() {
            Self::print_blue("══");
        }
        Self::print_blue("╗ ");
        print!("\n\r");

        for row in &grid {
            Self::print_blue("║ ");
            for cell in row {
                Self::draw_cell(&cell)
            }
            Self::print_blue("║ ");
            print!("\n\r");
        }

        // bottom box wall
        Self::print_blue("╚═");
        for _ in 0..grid[0].len() {
            Self::print_blue("══");
        }
        Self::print_blue("╝ ");
        print!("\n\r");

        //flush the output buffer
        std::io::stdout().flush().unwrap();
    }

    fn draw_cell(cell: &CellContent) {
        match cell {
            CellContent::Empty => print!("  "),

            CellContent::Head(orientation) => match orientation {
                Orientation::Up => Self::print_green("▲ "),
                Orientation::Right => Self::print_green("▶ "),
                Orientation::Down => Self::print_green("▼ "),
                Orientation::Left => Self::print_green(" ◀"),
            },

            CellContent::Tail(orientation) => match orientation {
                Orientation::Up => Self::print_green("╵ "),
                Orientation::Right => Self::print_green(" ╶"),
                Orientation::Down => Self::print_green("╷ "),
                Orientation::Left => Self::print_green("╴ "),
            },

            CellContent::Body { towards, from } => match (towards, from) {
                (Orientation::Up, Orientation::Down) => {
                    panic!("impossible snake orientation {:?}", (from, towards))
                }
                (Orientation::Up, Orientation::Left) => Self::print_green("┗━"),
                (Orientation::Up, Orientation::Up) => Self::print_green("┃ "),
                (Orientation::Up, Orientation::Right) => Self::print_green("┛ "),
                (Orientation::Right, Orientation::Down) => Self::print_green("┗━"),
                (Orientation::Right, Orientation::Left) => {
                    panic!("impossible snake orientation {:?}", (from, towards))
                }
                (Orientation::Right, Orientation::Up) => Self::print_green("┏━"),
                (Orientation::Right, Orientation::Right) => Self::print_green("━━"),
                (Orientation::Down, Orientation::Down) => Self::print_green("┃ "),
                (Orientation::Down, Orientation::Left) => Self::print_green("┏━"),
                (Orientation::Down, Orientation::Up) => {
                    panic!("impossible snake orientation {:?}", (from, towards))
                }
                (Orientation::Down, Orientation::Right) => Self::print_green("┓ "),
                (Orientation::Left, Orientation::Down) => Self::print_green("┛ "),
                (Orientation::Left, Orientation::Left) => Self::print_green("━━"),
                (Orientation::Left, Orientation::Up) => Self::print_green("┓ "),
                (Orientation::Left, Orientation::Right) => {
                    panic!("impossible snake orientation {:?}", (from, towards))
                }
            },
            CellContent::Apple => Self::print_red("● "),
        }
    }

    fn draw_game_over(score: usize) {
        print!("\n\r");
        print!("\n\r");
        Self::print_red(
            " ██████   █████  ███    ███ ███████      ██████  ██    ██ ███████ ██████  \r\n",
        );
        Self::print_red(
            "██       ██   ██ ████  ████ ██          ██    ██ ██    ██ ██      ██   ██ \r\n",
        );
        Self::print_red(
            "██   ███ ███████ ██ ████ ██ █████       ██    ██ ██    ██ █████   ██████  \r\n",
        );
        Self::print_red(
            "██    ██ ██   ██ ██  ██  ██ ██          ██    ██  ██  ██  ██      ██   ██ \r\n",
        );
        Self::print_red(
            " ██████  ██   ██ ██      ██ ███████      ██████    ████   ███████ ██   ██ \r\n",
        );
        print!("\n\r");
        print!("\n\r");
        println!("                  press enter to continue, esc to quit\r");
        println!("\r");
        Self::print_blue(&format!("                        your score is: {score}\r"));

        io::stdout().flush().expect("failed to flush")
    }

    fn draw_title_screen() {
        // clears the screen using ANSI escape codes
        print!("\x1B[2J\x1B[1;1H");

        println!("Welcome to:\r");
        print!("\n\r");
        print!("\n\r");
        Self::print_green("██████  ██    ██ ███████ ████████ ██    ██     ██████   █████  ████████ ████████ ██      ███████ \r\n");
        Self::print_green("██   ██ ██    ██ ██         ██     ██  ██      ██   ██ ██   ██    ██       ██    ██      ██      \r\n");
        Self::print_green("██████  ██    ██ ███████    ██      ████       ██████  ███████    ██       ██    ██      █████   \r\n");
        Self::print_green("██   ██ ██    ██      ██    ██       ██        ██   ██ ██   ██    ██       ██    ██      ██      \r\n");
        Self::print_green("██   ██  ██████  ███████    ██       ██        ██   ██ ██   ██    ██       ██    ███████ ███████ \r\n");
        print!("\n\r");
        print!("\n\r");
        println!("                                          controls:\r");
        println!("                                            wads to move\r");
        println!("                                            esc to quit\r");
        println!("");
        Self::print_blue("                                          press enter to continue\r");
    }

    fn print_green(s: &str) {
        print!("{GREEN}");
        print!("{s}");
        print!("{WHITE}");
    }
    fn print_red(s: &str) {
        print!("{RED}");
        print!("{s}");
        print!("{WHITE}");
    }
    fn print_blue(s: &str) {
        print!("{BLUE}");
        print!("{s}");
        print!("{WHITE}");
    }
}

mod tests;
