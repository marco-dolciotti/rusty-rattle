use super::CellContent;
use super::Orientation;

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
        ]
        .iter()
    }
}
