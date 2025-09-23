# Conway's Game of Life Implementation

## Rules from Wikipedia

- Any live cell with fewer than two live neighbours dies, as if by underpopulation.
- Any live cell with two or three live neighbours lives on to the next generation.
- Any live cell with more than three live neighbours dies, as if by overpopulation.
- Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

## Project Structure

```
.
└── src
    ├── grid.rs  # manages data as Vec<bool>
    └── main.rs  # start program

## Run and test

```

$ cargo run

```
![generation output](image.png "generation output")

```

$ cargo test -- --nocapture

```

```
