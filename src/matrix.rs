use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Matrix <T: Copy> {
    data: Vec<Vec<T>>
}

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
}

impl <T: Copy> Matrix <T> {
    pub fn new(rows: usize, cols: usize, value: T) -> Self {
        let mut data = Vec::with_capacity(rows);
        for _ in 0..rows {
            data.push(vec![value; cols]);
        }
        return Self {data};
    }

    pub fn data(&self, row: usize, col: usize) -> T {
        self.data[row][col]
    }

    pub fn set_data(&mut self, row: usize, col: usize, value: T) {
        self.data[row][col] = value;
    }

    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn cols(&self) -> usize {
        if let Some(ref first) = self.data.first() {
            first.len()
        } else {
            0
        }
    }

    pub fn row(&self, idx: usize) -> Vec<T> {
        self.data[idx].clone()
    }

    pub fn set_row(&mut self, idx: usize, row: &Vec<T>) {
        assert!(row.len() == self.cols());
        self.data[idx] = row.clone();
    }

    pub fn col(&self, idx: usize) -> Vec<T> {
        let mut col = Vec::new();
        for row in self.data.iter() {
            col.push(row[idx])
        }
        return col;
    }

    pub fn set_col(&mut self, idx: usize, col: &Vec<T>) {
        assert!(col.len() == self.rows());
        for (row_idx, ref mut row) in self.data.iter_mut().enumerate() {
            row[idx] = col[row_idx];
        }
    }

    pub fn get_random_by_pred<TPredicate>(&self, pred: TPredicate) -> Option<Cell>
    where TPredicate: Fn(T) -> bool {
        let mut selected = Vec::new();
        for (row_idx, &ref row) in self.data.iter().enumerate() {
            for (col_idx, &ref value) in row.iter().enumerate() {
                if pred(*value) {
                    let cell = Cell {
                        row: row_idx,
                        col: col_idx,
                    };
                    selected.push(cell);
                }
            }
        }
        return selected.choose(&mut rand::thread_rng()).copied();
    }
}
