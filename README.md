# Conway's Game of Life Implementation

## Rules from Wikipedia

- Any live cell with fewer than two live neighbours dies, as if by underpopulation.
- Any live cell with two or three live neighbours lives on to the next generation.
- Any live cell with more than three live neighbours dies, as if by overpopulation.
- Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

## Project Structure

```
.
|---first.rs double array for data matrix
|---second.rs refactoring count neighbors function using arrays
|---third.rs refactoring using Vec<Vec<bool>>
|---fourth.rs refacotring using Vec<bool> and grid struct


```
