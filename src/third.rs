fn next_generation(matrix: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut cloned = matrix.clone();
    for row in 0..matrix.len() {
        for column in 0..matrix[row].len() {
            let cell = matrix[row][column];
            let count = count_neighbors(matrix, row, column);
            if cell {
                if count < 2 {
                    cloned[row][column] = false;
                } else if count == 2 || count == 3 {
                    cloned[row][column] = true;
                } else if count > 3 {
                    cloned[row][column] = false;
                }
            } else {
                if count == 3 {
                    cloned[row][column] = true;
                }
            }
        }
    }
    cloned
}

fn count_neighbors(matrix: &Vec<Vec<bool>>, row: usize, column: usize) -> usize {
    let mut count = 0;
    let directions: [(i32, i32); 8] = [
        (0, -1),  // Left
        (-1, -1), // Up-left
        (-1, 0),  // Up
        (-1, 1),  // Up-right
        (0, 1),   // Right
        (1, 1),   // Bottom-right
        (1, 0),   // Bottom
        (1, -1),  // Bottom-left
    ];
    for (dr, dc) in directions {
        let new_row = row as i32 + dr;
        let new_column = column as i32 + dc;
        if new_row >= 0 && new_row < 5 && new_column >= 0 && new_column < 5 {
            if matrix[new_row as usize][new_column as usize] {
                count += 1;
            }
        }
    }
    count
}

fn show_generation(matrix: &Vec<Vec<bool>>, generation: usize) {
    let mut cloned = matrix.clone();
    for _ in 0..generation {
        cloned = next_generation(&cloned)
    }
    print_matrix(&cloned, 5, 5);
}

fn print_matrix(matrix: &Vec<Vec<bool>>, row: usize, column: usize) {
    for i in 0..row {
        let mut line: String = "".to_string();
        for j in 0..column {
            if matrix[i][j] {
                line.push_str("1 ");
            } else {
                line.push_str("0 ");
            }
        }
        println!("{line}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_neighbors_test() {
        let matrix = vec![
            vec![true, false, true, false, true],
            vec![true, true, true, false, true],
            vec![true, false, true, true, true],
            vec![false, false, false, false, true],
            vec![true, true, true, true, false],
        ];
        assert_eq!(count_neighbors(&matrix, 0, 0), 2);
        assert_eq!(count_neighbors(&matrix, 2, 2), 3);
        assert_eq!(count_neighbors(&matrix, 4, 4), 2);
    }

    #[test]
    fn count_matrix_test() {
        let matrix = vec![
            vec![true, false, true, false, true],
            vec![true, true, true, false, true],
            vec![true, false, true, true, true],
            vec![false, false, false, false, true],
            vec![true, true, true, true, false],
        ];
        let result = [
            [2, 5, 2, 4, 1],
            [3, 6, 4, 7, 3],
            [2, 5, 3, 5, 3],
            [3, 5, 5, 6, 3],
            [1, 2, 2, 2, 2],
        ];
        let mut counts = [[0usize; 5]; 5];
        for row in 0..5 {
            for column in 0..5 {
                counts[row][column] = count_neighbors(&matrix, row, column)
            }
        }
        assert_eq!(counts, result);
    }

    #[test]
    fn next_generation_test() {
        let matrix = vec![
            vec![true, false, true, false, true],
            vec![true, true, true, false, true],
            vec![true, false, true, true, true],
            vec![false, false, false, false, true],
            vec![true, true, true, true, false],
        ];
        let nextgen = [
            [true, false, true, false, false],
            [true, false, false, false, true],
            [true, false, true, false, true],
            [true, false, false, false, true],
            [false, true, true, true, false],
        ];
        println!("matrix");
        print_matrix(&matrix, 5, 5);
        println!("nexgen");
        print_matrix(&next_generation(&matrix), 5, 5);
        assert_eq!(next_generation(&matrix), nextgen);
    }

    #[test]
    fn show_generation_test() {
        let matrix = vec![
            vec![true, false, true, false, true],
            vec![true, true, true, false, true],
            vec![true, false, true, true, true],
            vec![false, false, false, false, true],
            vec![true, true, true, true, false],
        ];
        let generation = 20;
        println!("matrix");
        print_matrix(&matrix, 5, 5);
        println!("generation");
        show_generation(&matrix, generation);
    }
}
