fn next_generation(matrix: [[bool; 5]; 5]) -> [[bool; 5]; 5] {
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

fn count_neighbors(matrix: [[bool; 5]; 5], row: usize, column: usize) -> usize {
    let mut count = 0;
    if column > 0 {
        if matrix[row][column - 1] {
            count += 1;
        }
    }
    if column > 0 && row > 0 {
        if matrix[row - 1][column - 1] {
            count += 1;
        }
    }
    if row > 0 {
        if matrix[row - 1][column] {
            count += 1;
        }
    }
    if row > 0 && column < 5 - 1 {
        if matrix[row - 1][column + 1] {
            count += 1;
        };
    }
    if column < 5 - 1 {
        if matrix[row][column + 1] {
            count += 1;
        }
    }
    if row < 5 - 1 && column < 5 - 1 {
        if matrix[row + 1][column + 1] {
            count += 1;
        }
    }
    if row < 5 - 1 {
        if matrix[row + 1][column] {
            count += 1;
        }
    }
    if row < 5 - 1 && column > 0 {
        if matrix[row + 1][column - 1] {
            count += 1;
        }
    }
    count
}

fn show_neighbors(matrix: [[bool; 5]; 5], row: usize, column: usize) {
    let mut message = format!("[{row}][{column}]=");
    let mut buffer: String;
    if column > 0 {
        buffer = format!("left:{}, ", matrix[row][column - 1]);
        message.push_str(buffer.as_str());
    } else {
        message.push_str("left:nocell, ");
    }
    if column > 0 && row > 0 {
        buffer = format!("upleft:{}, ", matrix[row - 1][column - 1]);
        message.push_str(buffer.as_str());
    } else {
        message.push_str("upleft:nocell, ");
    }
    if row > 0 {
        buffer = format!("up:{}, ", matrix[row - 1][column]);
        message.push_str(buffer.as_str());
    } else {
        message.push_str("up:nocell, ");
    }
    if row > 0 && column < 5 - 1 {
        buffer = format!("upright:{}, ", matrix[row - 1][column + 1]);
        message.push_str(buffer.as_str());
    } else {
        message.push_str("upright:nocell, ");
    }
    if column < 5 - 1 {
        buffer = format!("right:{}, ", matrix[row][column + 1]);
        message.push_str(buffer.as_str());
    } else {
        message.push_str("right:nocell, ");
    }
    if row < 5 - 1 && column < 5 - 1 {
        buffer = format!("downright:{}, ", matrix[row + 1][column + 1]);
        message.push_str(buffer.as_str());
    } else {
        message.push_str("downright:nocell, ");
    }
    if row < 5 - 1 {
        buffer = format!("down:{}, ", matrix[row + 1][column]);
        message.push_str(buffer.as_str());
    } else {
        message.push_str("down:nocell, ");
    }
    if row < 5 - 1 && column > 0 {
        buffer = format!("downleft:{}", matrix[row + 1][column - 1]);
        message.push_str(buffer.as_str());
    } else {
        message.push_str("downleft:nocell");
    }
    println!("{message}");
}

fn show_generation(matrix: [[bool; 5]; 5], generation: usize) {
    let mut cloned = matrix.clone();
    for _ in 0..generation {
        cloned = next_generation(cloned)
    }
    print_matrix(cloned, 5, 5);
}

fn print_matrix(matrix: [[bool; 5]; 5], row: usize, column: usize) {
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

    //#[test]
    fn show_neighbors_test() {
        let matrix = [
            [true, false, true, false, true],
            [true, true, true, false, true],
            [true, false, true, true, true],
            [false, false, false, false, true],
            [true, true, true, true, false],
        ];
        show_neighbors(matrix, 0, 0);
        show_neighbors(matrix, 2, 2);
        show_neighbors(matrix, 4, 4);
        show_neighbors(matrix, 0, 1);
    }

    //#[test]
    fn count_neighbors_test() {
        let matrix = [
            [true, false, true, false, true],
            [true, true, true, false, true],
            [true, false, true, true, true],
            [false, false, false, false, true],
            [true, true, true, true, false],
        ];
        assert_eq!(count_neighbors(matrix, 0, 0), 2);
        assert_eq!(count_neighbors(matrix, 2, 2), 3);
        assert_eq!(count_neighbors(matrix, 4, 4), 2);
    }

    //#[test]
    fn count_matrix_test() {
        let matrix = [
            [true, false, true, false, true],
            [true, true, true, false, true],
            [true, false, true, true, true],
            [false, false, false, false, true],
            [true, true, true, true, false],
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
                counts[row][column] = count_neighbors(matrix, row, column)
            }
        }
        assert_eq!(counts, result);
    }

    //#[test]
    fn next_generation_test() {
        let matrix = [
            [true, false, true, false, true],
            [true, true, true, false, true],
            [true, false, true, true, true],
            [false, false, false, false, true],
            [true, true, true, true, false],
        ];
        let nextgen = [
            [true, false, true, false, false],
            [true, false, false, false, true],
            [true, false, true, false, true],
            [true, false, false, false, true],
            [false, true, true, true, false],
        ];
        println!("matrix");
        print_matrix(matrix, 5, 5);
        println!("nexgen");
        print_matrix(next_generation(matrix), 5, 5);
        assert_eq!(next_generation(matrix), nextgen);
    }

    #[test]
    fn show_generation_test() {
        let matrix = [
            [true, false, true, false, true],
            [true, true, true, false, true],
            [true, false, true, true, true],
            [false, false, false, false, true],
            [true, true, true, true, false],
        ];
        let generation = 20;
        println!("matrix");
        print_matrix(matrix, 5, 5);
        println!("generation");
        show_generation(matrix, generation);
    }
}
