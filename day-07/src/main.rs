use tools::read_input_file;

fn main() {
    // Read the entire input as a string
    let input = read_input_file();

    // Convert the input into a 2D grid of characters
    // grid[row][col]
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    // ------------------------------------------------------------
    // ways[r][c] = number of timelines that reach cell (r, c)
    //
    // This is the dynamic-programming table.
    // We do NOT modify the grid; we only accumulate counts.
    // ------------------------------------------------------------
    let mut ways = vec![vec![0i64; cols]; rows];

    // ------------------------------------------------------------
    // Initialization:
    // Find the starting point 'S' and set its count to 1.
    //
    // There is exactly one timeline before the particle starts.
    // ------------------------------------------------------------
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 'S' {
                ways[r][c] = 1;
            }
        }
    }

    // ------------------------------------------------------------
    // Process the grid top-to-bottom.
    //
    // This works because the grid is a DAG:
    // all transitions go from row r to row r+1.
    // ------------------------------------------------------------
    for r in 0..rows - 1 {
        for c in 0..cols {
            let current_ways = ways[r][c];

            // If no timelines reach this cell, skip it.
            if current_ways == 0 {
                continue;
            }

            // Look at the cell directly below
            match grid[r + 1][c] {
                // ------------------------------------------------
                // Empty space:
                // The particle continues straight down.
                // All timelines go to (r+1, c).
                // ------------------------------------------------
                '.' => {
                    ways[r + 1][c] += current_ways;
                }

                // ------------------------------------------------
                // Splitter:
                // Time splits into two timelines.
                // One goes down-left, one goes down-right.
                // ------------------------------------------------
                '^' => {
                    // Down-left
                    if c > 0 {
                        ways[r + 1][c - 1] += current_ways;
                    }

                    // Down-right
                    if c + 1 < cols {
                        ways[r + 1][c + 1] += current_ways;
                    }
                }

                // ------------------------------------------------
                // Any other character (shouldn't normally happen)
                // ------------------------------------------------
                _ => {}
            }
        }
    }

    // ------------------------------------------------------------
    // The particle exits the manifold from the bottom row.
    //
    // Each exit position represents a completed timeline.
    // Even if multiple timelines exit at the same column,
    // they are still distinct histories and must be counted.
    // ------------------------------------------------------------
    let total_timelines: i64 = ways[rows - 1].iter().sum();

    println!("timelines: {}", total_timelines);
}
