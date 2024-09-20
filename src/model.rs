use std::{cell::Cell, sync::mpsc, thread};

use crate::{views::{tui_view::TuiView, View}, Config, ViewType};

pub const GRID_HEIGHT: usize = 30;
pub const GRID_WIDTH: usize = 40;

#[derive(Clone, Copy, Debug)]
pub enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
pub enum CellContent {
    Empty,
    Head(Orientation),
    Tail(Orientation),
    Body {
        towards: Orientation,
        from: Orientation,
    },
    Apple,
}



pub enum ModelToViewMessage {
    UpdateFrame([[CellContent; GRID_WIDTH]; GRID_HEIGHT]),
    Quit,
}

pub struct Model {
    grid: [[CellContent; GRID_WIDTH]; GRID_HEIGHT],
    /// to send messages to the Drawer
    sender: mpsc::Sender<ModelToViewMessage>,

}

impl Model {
    pub fn new(sender: mpsc::Sender<ModelToViewMessage>) -> Self {

        Model {
            grid: [[CellContent::Empty; GRID_WIDTH]; GRID_HEIGHT],
            sender,
        }
    }
}


impl<'a> IntoIterator for &'a CellContent {
    type Item = &'a CellContent;

    type IntoIter = std::slice::Iter<'a, CellContent>;

    fn into_iter(self) -> Self::IntoIter {
        [
            CellContent::Apple,

            CellContent::Empty,

            CellContent::Head(Orientation::Up),
            CellContent::Head(Orientation::Right),
            CellContent::Head(Orientation::Down),
            CellContent::Head(Orientation::Left),

            CellContent::Tail(Orientation::Up),
            CellContent::Tail(Orientation::Right),
            CellContent::Tail(Orientation::Down),
            CellContent::Tail(Orientation::Left),


            CellContent::Body {
                towards: Orientation::Up,
                from: Orientation::Up,
            },
            CellContent::Body {
                towards: Orientation::Right,
                from: Orientation::Right,
            },
            CellContent::Body {
                towards: Orientation::Down,
                from: Orientation::Down,
            },
            CellContent::Body {
                towards: Orientation::Left,
                from: Orientation::Left,
            },
            CellContent::Body {
                towards: Orientation::Up,
                from: Orientation::Right,
            },
            CellContent::Body {
                towards: Orientation::Up,
                from: Orientation::Left,
            },
            CellContent::Body {
                towards: Orientation::Up,
                from: Orientation::Up,
            },
            CellContent::Body {
                towards: Orientation::Right,
                from: Orientation::Down,
            },
            CellContent::Body {
                towards: Orientation::Right,
                from: Orientation::Up,
            },
            CellContent::Body {
                towards: Orientation::Right,
                from: Orientation::Right,
            },
            CellContent::Body {
                towards: Orientation::Down,
                from: Orientation::Left,
            },
            CellContent::Body {
                towards: Orientation::Down,
                from: Orientation::Right,
            },
            CellContent::Body {
                towards: Orientation::Down,
                from: Orientation::Down,
            },
            CellContent::Body {
                towards: Orientation::Left,
                from: Orientation::Up,
            },
            CellContent::Body {
                towards: Orientation::Left,
                from: Orientation::Down,
            },
            CellContent::Body {
                towards: Orientation::Left,
                from: Orientation::Left,
            },


        ].iter()
    }
}