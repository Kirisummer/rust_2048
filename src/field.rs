use std::fmt;
use rand::seq::SliceRandom;
use either;

use crate::matrix::Matrix;

pub type Value = u64;
pub type Score = u64;

#[derive(Debug)]
pub struct Field {
    matrix: Matrix<Value>,
}

#[derive(Copy, Clone)]
pub enum ShiftDim {
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone)]
pub enum ShiftDir {
    Direct,
    Reverse,
}

impl Field {
    const EMPTY: Value = 0;
    // array constant as a reference to an array with static lifetime
    const RANDOM_NUMBERS: &'static [Value] = &[2, 4];

    pub fn new(size: usize) -> Self {
        let matrix = Matrix::new(size, size, Self::EMPTY);
        return Self {matrix};
    }

    pub fn matrix(&self, row: usize, col: usize) -> Value {
        self.matrix.data(row, col)
    }

    pub fn add_random_cell (&mut self) -> bool {
        let random = self.matrix
            .get_random_by_pred(Self::is_empty);
        match random {
            Some(cell) => {
                let value = Self::get_random_value();
                self.matrix.set_data(cell.row, cell.col, value);
                true
            },
            None => false
        }
    }

    pub fn shift(&mut self, dim: ShiftDim, dir: ShiftDir) -> Option<Score> {
        type Get = fn(&Matrix<Value>, usize) -> Vec<Value>;
        type Set = fn(&mut Matrix<Value>, usize, &Vec<Value>);

        let size = self.matrix.rows(); // == _.cols()
        let (get, set): (Get, Set) = match dim {
            ShiftDim::Horizontal => (Matrix::row, Matrix::set_row),
            ShiftDim::Vertical => (Matrix::col, Matrix::set_col),
        };

        let mut total_points: Score = 0;
        let mut moved = false;
        for idx in 0..size {
            let vec = get(&self.matrix, idx);
            let (new, points) = Self::shift_vec(&vec, dir);
            // check if vector changed (one or more cells changed)
            if vec.iter().zip(&new).filter(|&(a, b)| a != b).count() > 0 {
                set(&mut self.matrix, idx, &new);
                total_points += points;
                moved = true;
            }
        }

        if moved {
            Some(total_points)
        } else {
            None
        }
    }

    fn is_empty(value: Value) -> bool {
        value == Self::EMPTY
    }

    fn get_random_value() -> Value {
        Self::RANDOM_NUMBERS.choose(&mut rand::thread_rng())
                            .copied()
                            .expect("Failed to get random value")
    }

    fn shift_vec(vec: &Vec<Value>, dir: ShiftDir) -> (Vec<Value>, Score) {
        let mut new = Vec::new();
        let mut prev_squashed = false;
        let mut points: Score = 0;

        let vec_iter = match dir {
            ShiftDir::Direct => either::Left(vec.iter()),
            ShiftDir::Reverse => either::Right(vec.iter().rev()),
        };

        for &value in vec_iter.into_iter() {
            if value == Self::EMPTY {
                continue;
            }

            if let Some(last) = new.last_mut() {
                if !prev_squashed && value == *last {
                    *last <<= 1;
                    points += *last;
                    prev_squashed = true;
                } else {
                    new.push(value);
                    prev_squashed = false;
                }
            } else {
                new.push(value);
                prev_squashed = false;
            }
        }

        Self::pad_vec(&mut new, vec.len());
        match dir {
            ShiftDir::Reverse => new.reverse(),
            ShiftDir::Direct => (),
        }

        return (new, points);
    }

    fn pad_vec(vec: &mut Vec<Value>, size: usize) {
        assert!(vec.len() <= size);
        vec.reserve_exact(size - vec.len());
        while vec.len() < size {
            vec.push(Self::EMPTY);
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut fmt = String::new();
        for row in 0..self.matrix.rows() {
            for col in 0..self.matrix.cols() {
                let value = self.matrix.data(row, col);
                fmt.push_str(&value.to_string());
                fmt.push(' ');
            }
            fmt.push('\n');
        }
        write!(f, "{}", fmt.as_str())
    }
}
