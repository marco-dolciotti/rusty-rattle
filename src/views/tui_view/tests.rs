#[allow(unused_imports)]
use std::{thread, time::Duration};

#[allow(unused_imports)]
use crate::{model::{CellContent, Orientation}, views::tui_view::TuiView};

#[test]
fn test_draw_cell() {
    //to not overlap it with test_draw_grid()
    thread::sleep(Duration::from_secs(2));

    print!("\n\n\n");
    CellContent::Empty.into_iter().for_each(|cell| TuiView::draw_cell(&cell));
    print!("\n\n\n");
}

#[test]
fn test_draw_grid() {
    use super::TuiView;
    use crate::model::{CellContent, GRID_HEIGHT, GRID_WIDTH};

    let mut grid = core::array::from_fn(|_| 
                                            core::array::from_fn(|_| 
                                                CellContent::default()));
    grid[1][1] = CellContent::Apple;
    grid[10][10] = CellContent::Head(Orientation::Up);
    grid[11][10] = CellContent::Body {
        towards: Orientation::Up,
        from: Orientation::Up,
    };
    grid[12][10] = CellContent::Body {
        towards: Orientation::Up,
        from: Orientation::Right,
    };
    grid[12][9] = CellContent::Body {
        towards: Orientation::Right,
        from: Orientation::Right,
    };
    grid[12][8] = CellContent::Body {
        towards: Orientation::Right,
        from: Orientation::Down,
    };
    grid[11][8] = CellContent::Body {
        towards: Orientation::Down,
        from: Orientation::Down,
    };
    grid[10][8] = CellContent::Body {
        towards: Orientation::Down,
        from: Orientation::Down,
    };
    grid[9][8] = CellContent::Body {
        towards: Orientation::Down,
        from: Orientation::Left,
    };
    grid[9][9] = CellContent::Body {
        towards: Orientation::Left,
        from: Orientation::Left,
    };
    grid[9][10] = CellContent::Body {
        towards: Orientation::Left,
        from: Orientation::Left,
    };
    grid[9][11] = CellContent::Body {
        towards: Orientation::Left,
        from: Orientation::Left,
    };
    grid[9][12] = CellContent::Body {
        towards: Orientation::Left,
        from: Orientation::Up,
    };
    grid[10][12] = CellContent::Body {
        towards: Orientation::Up,
        from: Orientation::Up,
    };
    grid[11][12] = CellContent::Tail(Orientation::Up);
    TuiView::draw_grid(grid);
}
