use std::{env, fs, path};

fn main() {
    let input_string = read_input_file();
    let mut grid: Vec<Vec<char>> =
        input_string.lines().map(|line| line.chars().collect()).collect();

    let mut total_removed = 0;

    loop {
        let mut to_remove = Vec::new();

        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                if grid[r][c] != '@' {
                    continue;
                }

                let mut neighbors = 0;

                let row_range = r.saturating_sub(1)..=(r + 1).min(grid.len() - 1);
                let col_range = c.saturating_sub(1)..=(c + 1).min(grid[r].len() - 1);

                for rr in row_range.clone() {
                    for cc in col_range.clone() {
                        if (rr != r || cc != c) && grid[rr][cc] == '@' {
                            neighbors += 1;
                        }
                    }
                }

                if neighbors < 4 {
                    to_remove.push((r, c));
                }
            }
        }

        if to_remove.is_empty() {
            break; // no more accessible rolls
        }

        // Remove all marked rolls
        for (r, c) in to_remove.clone() {
            grid[r][c] = '.';
        }

        total_removed += to_remove.len() as u32;
    }

    println!("Total removable rolls: {}", total_removed);
}

fn read_input_file() -> String {
    let path = format!(
        "{}{}src{}input.txt",
        env::current_dir()
            .expect("Failed to get current directory")
            .to_str()
            .expect("Failed to convert path to string"),
        path::MAIN_SEPARATOR,
        path::MAIN_SEPARATOR
    );

    let contents_with_bom =
        fs::read_to_string(&path).expect(&format!("Failed to read file: {}", path));
    contents_with_bom.trim_start_matches('\u{FEFF}').to_string()
}
