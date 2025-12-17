use tools::read_input_file;
use tools::write_result_file;

fn main() {
    let input = read_input_file();
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    if grid.is_empty() {
        return;
    }

    for row in 0..grid.len() -1 {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'S' {
                grid[row+1][col] = '|';
            }
            if grid[row][col] == '|' && row < grid.len() && grid[row+1][col] == '.'{
                grid[row+1][col] = '|';
            }
            if grid[row][col] == '|' && row < grid.len() && grid[row+1][col] == '^'{
                grid[row+1][col-1] = '|';
                grid[row+1][col+1] = '|';
            }
        }
    }

    let mut count: i64 = 0;

    for row in 1..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '^' && grid[row-1][col] == '|' {
               count += 1;
            }
        }
    }
    println!("count: {}", count);
    
    let mut result: String = String::new();
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            result.push(grid[row][col]);
        }
        result.push('\n');
    }

    write_result_file(&result);


}
