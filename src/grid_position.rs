//! A module to help the use of coordinate

use std::cmp::{max, min};
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPosition {
    pub(crate) x: i16,
    pub(crate) y: i16,
}

impl GridPosition {
    /// We make a standard helper function so that we can create a new `GridPosition` more easily.
    pub fn new(x: i16, y: i16) -> Self {
        GridPosition { x, y }
    }

    /// Create a GridPosition from a float coordinate
    pub fn from_screen(x: f32, y: f32) -> Self {
        GridPosition { x: x as i16, y: y as i16 }
    }

    /// Verify if the GridPosition is in the screen
    pub fn in_screen(&self) -> bool {
        0 < self.x && self.x < SCREEN_SIZE.0 as i16 &&
            0 < self.y && self.y < SCREEN_SIZE.1 as i16
    }

    /// Convert the ScreenPosition (pixel) into GridPosition
    pub fn into_grid(self) -> Self {
        let x = self.x / GRID_CELL_SIZE.0;
        let y = self.y / GRID_CELL_SIZE.1;
        GridPosition { x, y }
    }

    /// Return the max of absolut coordinates
    pub fn max(&self) -> i16 {
        max(self.x.abs(), self.y.abs())
    }

    /// Return the min of absolut coordinates
    pub fn min(&self) -> i16 {
        min(self.x.abs(), self.y.abs())
    }
}

impl Add for GridPosition {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        GridPosition::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for GridPosition {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        GridPosition::new(self.x - other.x, self.y - other.y)
    }
}

impl AddAssign for GridPosition {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign for GridPosition {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl fmt::Display for GridPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
