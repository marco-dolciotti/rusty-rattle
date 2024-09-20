use std::env;
use std::sync::mpsc;

use rusty_rattle::controller::Controller;
use rusty_rattle::model::{self, Model};
use rusty_rattle::views::{self, tui_view};
use rusty_rattle::Config;

fn main() {
    let config = Config::new(&mut env::args());

    let (sender, receiver) = mpsc::channel();

    let mut model = Model::new(sender);
    let controller = Controller::new(model);
    let view = views::new_view(config, controller, receiver);

    view.run();

}
