use std::{env, thread};
use std::sync::mpsc;

use rusty_rattle::controller::Controller;
use rusty_rattle::model::Model;
use rusty_rattle::views;
use rusty_rattle::Config;
use rusty_rattle::event_processes;

fn main() {
    let config = Config::new(&mut env::args());

    let (event_sender, event_receiver) = mpsc::channel();

    let view = views::new_view(config);
    let model = Model::new(view);
    let controller = Controller::new(model);

    let event_sender_clone1 = event_sender.clone();
    let event_sender_clone2 = event_sender.clone();
    let handles = [
        thread::spawn(|| event_processes::event_loop(event_receiver, controller)),
        thread::spawn(move || event_processes::game_tick_loop(event_sender_clone1)),
        thread::spawn(move || event_processes::input_loop(event_sender_clone2))
    ];

    for handle in handles {
        handle.join().unwrap();
    }
}
