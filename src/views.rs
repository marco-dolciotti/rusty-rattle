use std::sync::mpsc;

use tui_view::TuiView;

use crate::{controller::Controller, model::ModelToViewMessage, Config, ViewType};

pub mod tui_view;

pub trait View {
    fn run(&self, receiver: mpsc::Receiver<ModelToViewMessage>);
    fn set_controller(&mut self, controller: Controller);
}

pub fn new_view(config: Config, controller: Controller, receiver: mpsc::Receiver<ModelToViewMessage> ) -> Box<dyn View> {
    match config.view_type() {
        ViewType::GUI => todo!(),
        ViewType::TUI => Box::new(TuiView::new(controller, receiver))
    }
}
