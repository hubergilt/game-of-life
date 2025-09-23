#[derive(Clone)]
pub struct Grid {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<bool>,
}

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            cols,
            rows,
            data: vec![false; rows * cols],
        }
    }

    pub fn get(&self, row: usize, col: usize) -> bool {
        self.data[self.cols * row + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: bool) {
        self.data[self.cols * row + col] = value;
    }

    pub fn from_vec(&mut self, data: Vec<i32>) {
        for index in 0..data.len() {
            if data[index] == 1 {
                self.data[index] = true;
            } else if data[index] == 0 {
                self.data[index] = false;
            }
        }
    }

    fn count_neighbors(&self, row: usize, col: usize) -> usize {
        let mut count = 0;
        let directions: [(i32, i32); 8] = [
            (0, -1),  // Left
            (-1, -1), // Up-left
            (-1, 0),  // Up
            (-1, 1),  // Up-right
            (0, 1),   // Right
            (1, 1),   // Bottom-right
            (1, 0),   // Bottomg
            (1, -1),  // Bottom-left
        ];
        for (dr, dc) in directions {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;
            let valid_row = new_row < self.rows as i32 && new_row >= 0;
            let valid_col = new_col < self.cols as i32 && new_col >= 0;
            if valid_row && valid_col {
                if self.get(new_row as usize, new_col as usize) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn next_generation(&self) -> Self {
        let mut nextgen = Self::new(self.rows, self.cols);
        for row in 0..self.rows {
            for col in 0..self.cols {
                let cell = self.get(row, col);
                let count = self.count_neighbors(row, col);
                if cell {
                    if count < 2 {
                        nextgen.set(row, col, false);
                    } else if count == 2 || count == 3 {
                        nextgen.set(row, col, true);
                    } else if count > 3 {
                        nextgen.set(row, col, false);
                    }
                } else {
                    if count == 3 {
                        nextgen.set(row, col, true);
                    }
                }
            }
        }
        nextgen
    }

    #[allow(dead_code)]
    fn nth_generation(&self, generation: usize) -> Self {
        let mut nthgen = Self::new(self.rows, self.cols);
        nthgen.data = self.data.clone();
        for _ in 0..generation {
            nthgen = nthgen.next_generation()
        }
        nthgen
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for row in 0..self.rows {
            let mut line = String::new();
            for col in 0..self.cols {
                if self.get(row, col) {
                    line.push_str("1 ");
                } else {
                    line.push_str("0 ");
                }
            }
            println!("{}", line);
        }
    }

    pub fn format_grid(&self) -> String {
        let mut result = String::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.get(row, col) {
                    result.push_str("██");
                } else {
                    result.push_str("░░");
                }
            }
            result.push('\n');
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_neighbors_test() {
        let data = vec![
            true, false, true, false, true, //
            true, true, true, false, true, //
            true, false, true, true, true, //
            false, false, false, false, true, //
            true, true, true, true, false, //
        ]; //
        let mut grid = Grid::new(5, 5);
        grid.data = data;
        assert_eq!(grid.count_neighbors(0, 0), 2);
        assert_eq!(grid.count_neighbors(2, 2), 3);
        assert_eq!(grid.count_neighbors(4, 4), 2);
    }

    #[test]
    fn count_data_test() {
        let data = vec![
            true, false, true, false, true, //
            true, true, true, false, true, //
            true, false, true, true, true, //
            false, false, false, false, true, //
            true, true, true, true, false, //
        ]; //
        let mut grid = Grid::new(5, 5);
        grid.data = data;
        let result = [
            2, 5, 2, 4, 1, //
            3, 6, 4, 7, 3, //
            2, 5, 3, 5, 3, //
            3, 5, 5, 6, 3, //
            1, 2, 2, 2, 2, //
        ];
        let mut counts = [0usize; 25];
        for row in 0..5 {
            for column in 0..5 {
                counts[grid.cols * row + column] = grid.count_neighbors(row, column)
            }
        }
        assert_eq!(counts, result);
    }

    #[test]
    fn next_generation_test() {
        let data = vec![
            true, false, true, false, true, //
            true, true, true, false, true, //
            true, false, true, true, true, //
            false, false, false, false, true, //
            true, true, true, true, false, //
        ];
        let mut grid = Grid::new(5, 5);
        grid.data = data;
        let nextgen = [
            true, false, true, false, false, //
            true, false, false, false, true, //
            true, false, true, false, true, //
            true, false, false, false, true, //
            false, true, true, true, false, //
        ];
        assert_eq!(grid.next_generation().data, nextgen);
    }

    #[test]
    fn show_generation_test() {
        let data = vec![
            true, false, true, false, true, //
            true, true, true, false, true, //
            true, false, true, true, true, //
            false, false, false, false, true, //
            true, true, true, true, false, //
        ];
        let mut grid = Grid::new(5, 5);
        grid.data = data;
        grid.print();
        grid.next_generation().print();
    }
}
