use std::io::Write;
use std::sync::mpsc;
use std::time::Duration;
use std::{io, thread};

use rusty_rattle::controller::Controller;
use rusty_rattle::event_processes;
use rusty_rattle::model::Model;
use rusty_rattle::{views, Config, ViewType};

fn main() {
    let config = initialize_config();

    let (event_sender, event_receiver) = mpsc::channel();

    let view = views::new_view(config);
    let model = Model::new(view, config);
    let controller = Controller::new(model);

    let event_sender_clone1 = event_sender.clone();
    let event_sender_clone2 = event_sender.clone();
    let handles = [
        thread::spawn(|| event_processes::event_loop(event_receiver, controller)),
        thread::spawn(move || event_processes::game_tick_loop(event_sender_clone1, config)),
        thread::spawn(move || event_processes::input_loop(event_sender_clone2)),
    ];

    for handle in handles {
        handle.join().unwrap();
    }
}

fn initialize_config() -> Config {
    let stdin = io::stdin();

    let view_type = ViewType::TUI;

    print!("choose the grid height:");
    io::stdout().flush().expect("failed to flush stdout");
    let mut grid_height = String::new();
    stdin
        .read_line(&mut grid_height)
        .expect("failed to read line");
    let grid_height = grid_height.trim().parse().unwrap_or(20);

    print!("choose the grid width:");
    io::stdout().flush().expect("failed to flush stdout");
    let mut grid_width = String::new();
    stdin
        .read_line(&mut grid_width)
        .expect("failed to read line");
    let grid_width = grid_width.trim().parse().unwrap_or(30);

    print!("choose the time interval between steps (in milliseconds):");
    io::stdout().flush().expect("failed to flush stdout");
    let mut update_interval = String::new();
    stdin
        .read_line(&mut update_interval)
        .expect("failed to read line");
    let update_interval = update_interval.trim().parse().unwrap_or(200);
    let update_interval = Duration::from_millis(update_interval);

    let config = Config::builder()
        .set_view_type(view_type)
        .set_grid_height(grid_height)
        .set_grid_width(grid_width)
        .set_update_interval(update_interval)
        .build();
    config
}
