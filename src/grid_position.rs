
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GridPosition {
    x: i16,
    y: i16,
}

impl GridPosition {
    /// We make a standard helper function so that we can create a new `GridPosition` more easily.
    pub fn new(x: i16, y: i16) -> Self {
        GridPosition { x, y }
    }
}
