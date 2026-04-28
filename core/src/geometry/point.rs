//! A point in 2D space.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Add, Div, Index, Mul, Sub};

/// A point in 2D space.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    /// The x-coordinate of the point.
    pub x: f64,
    /// The y-coordinate of the point.
    pub y: f64,
}

/// Create a new `Point` struct.
impl Point {
    /// Create a new `Point` instance with given x and y coordinates.
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Implement common mathematical operations for `Point`.
impl Add for Point {
    type Output = Self;
    fn add(self, addend: Self) -> Self {
        Self {
            x: self.x + addend.x,
            y: self.y + addend.y,
        }
    }
}

/// Implement common mathematical operations for `Point`.
impl Sub for Point {
    type Output = Self;
    fn sub(self, subtrahend: Self) -> Self {
        Self {
            x: self.x - subtrahend.x,
            y: self.y - subtrahend.y,
        }
    }
}

/// Implement common mathematical operations for `Point`.
impl Mul<f64> for Point {
    type Output = Self;
    fn mul(self, factor: f64) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

/// Implement common mathematical operations for `Point`.
impl Div<f64> for Point {
    type Output = Self;
    fn div(self, denominator: f64) -> Self {
        Self {
            x: self.x / denominator,
            y: self.y / denominator,
        }
    }
}

/// Implement `Display` for `Point`.
impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Implement indexing for `Point`.
impl Index<usize> for Point {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// Implement conversion from tuple to `Point`.
impl From<(f64, f64)> for Point {
    fn from(tuple: (f64, f64)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

/// Implement conversion from `Point` to tuple.
impl From<Point> for (f64, f64) {
    fn from(point: Point) -> Self {
        (point.x, point.y)
    }
}
