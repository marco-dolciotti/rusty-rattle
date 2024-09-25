use crate::model::Orientation;

fn test_draw_cell() {
    use super::TuiView;
    use crate::model::{CellContent, Orientation};

    print!("\n\n\n");
    CellContent::Empty.into_iter().for_each(|cell| TuiView::draw_cell(&cell));
    print!("\n\n\n");
}

#[test]
fn test_draw_grid() {
    use super::TuiView;
    use crate::model::{CellContent, GRID_HEIGHT, GRID_WIDTH};

    let mut grid = [[CellContent::Empty; GRID_WIDTH]; GRID_HEIGHT];
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
