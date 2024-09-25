pub mod controller;
pub mod model;
pub mod views;
pub mod event_processes;

pub struct Config {
    view_type: ViewType,
}

impl Config {
    pub fn view_type(&self) -> &ViewType {
        &self.view_type
    }

    pub fn new<T>(args:&mut T) -> Self
        where
            T: Iterator<Item = String>
    {
        let view_type = match args.find(|arg| arg == "--gui") {
            Some(_) => ViewType::GUI,
            None => ViewType::TUI,
        };

        Config { view_type }
    }
}

#[derive(Clone)]
enum ViewType {
    GUI,
    TUI,
}
