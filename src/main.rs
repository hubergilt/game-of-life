use game_of_life::Grid;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

fn main() {
    #[rustfmt::skip]
    let data = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0,
        0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0,
        0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0,
        0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0,
        0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0,
        0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0,
        0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let mut grid = Grid::new(15, 15);
    let generation = 16;
    grid.from_vec(data);
    println!("=== Conway's Game of Life Animation ===\n");
    println!("Initial state:");
    grid.print();
    println!();

    println!("🎮 Live grid animation...");

    let current_grid = loop_display(&grid, generation);
    let grid_display = current_grid.format_grid();

    println!("Animation complete! Final generation: {generation}");
    println!("{grid_display}");

    current_grid.print();

    println!("\n{}", "=".repeat(30));
}

fn loop_display(initial_grid: &Grid, generations: usize) -> Grid {
    let pb = ProgressBar::new(generations as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "\n{spinner:.green} Generation {pos}/{len} [{elapsed_precise}] [{wide_bar:.cyan/blue}]\n{wide_msg}"
        )
        .unwrap()
        .progress_chars("⣾⣽⣻⢿⡿⣟⣯⣷")
    );

    let mut current_grid = initial_grid.clone();

    for i in 0..generations {
        let grid_display = current_grid.format_grid();
        pb.set_message(grid_display);
        pb.set_position(i as u64);

        current_grid = current_grid.next_generation();
        thread::sleep(Duration::from_millis(300));
    }
    pb.finish_and_clear();
    current_grid
}
