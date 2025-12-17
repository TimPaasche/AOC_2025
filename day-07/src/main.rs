use tools::read_input_file;

fn main() {
    let input = read_input_file();
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut ways = vec![vec![0i64; cols]; rows];

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 'S' {
                ways[r][c] = 1;
            }
        }
    }

    // Process top-down (DAG order)
    for r in 0..rows - 1 {
        for c in 0..cols {
            let w = ways[r][c];
            if w == 0 {
                continue;
            }

            match grid[r + 1][c] {
                '.' => {
                    ways[r + 1][c] += w;
                }
                '^' => {
                    if c > 0 {
                        ways[r + 1][c - 1] += w;
                    }
                    if c + 1 < cols {
                        ways[r + 1][c + 1] += w;
                    }
                }
                _ => {}
            }
        }
    }

    let total: i64 = ways[rows - 1].iter().sum();
    println!("timelines: {}", total);
}
