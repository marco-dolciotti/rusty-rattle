use std::sync::mpsc;

use tui_view::TuiView;

use crate::{controller::Controller, model::{CellContent, GRID_HEIGHT, GRID_WIDTH}, Config, ViewType};

pub mod tui_view;

pub trait View: Send{
    fn draw_title_screen(&self);
    fn draw_frame(&self, grid: [[CellContent; GRID_WIDTH]; GRID_HEIGHT]);
    fn draw_game_over(&self);
}

pub fn new_view(config: Config) -> Box<dyn View> {
    match config.view_type() {
        ViewType::GUI => todo!(),
        ViewType::TUI => Box::new(TuiView::new())
    }
}
