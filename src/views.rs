use tui_view::TuiView;

use crate::{model::CellContent, Config, ViewType};

pub mod tui_view;

pub trait View: Send {
    fn draw_title_screen(&self);
    fn draw_frame(&self, grid: Vec<Vec<CellContent>>);
    fn draw_game_over(&self, score: usize);
}

pub fn new_view(config: Config) -> Box<dyn View> {
    match config.view_type() {
        ViewType::GUI => todo!(),
        ViewType::TUI => Box::new(TuiView::new()),
    }
}
