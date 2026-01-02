use tools::read_input_file;

#[derive(Debug, Clone)]
struct MathProblem {
    numbers: Vec<i64>,
    operation: Operation,
}

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    None,
    Plus,
    Multiply,
}

impl MathProblem {
    fn solve(&self) -> i64 {
        match self.operation {
            Operation::Plus => self.numbers.iter().sum(),
            Operation::Multiply => self.numbers.iter().product(),
            Operation::None => self.numbers.first().copied().unwrap_or(0),
        }
    }
}

fn main() {
    let input = read_input_file();
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    if grid.is_empty() {
        return;
    }

    let mut problems = Vec::new();
    let mut current = MathProblem {
        numbers: vec![],
        operation: Operation::None,
    };

    // let row_count = grid.len();
    let col_count = grid[0].len();

    for x in 0..col_count {
        let has_data = (0..=3).any(|y| {
            grid.get(y)
                .and_then(|row| row.get(x))
                .map_or(false, |c| c.is_alphanumeric())
        });

        if !has_data {
            if !current.numbers.is_empty() {
                problems.push(current);
                current = MathProblem {
                    numbers: vec![],
                    operation: Operation::None,
                };
            }
            continue;
        }

        let col_string: String = (0..=3).filter_map(|y| grid.get(y)?.get(x)).collect();

        if let Ok(num) = col_string.trim().parse::<i64>() {
            current.numbers.push(num);
        }

        if let Some(op_row) = grid.get(4) {
            match op_row.get(x) {
                Some('+') => current.operation = Operation::Plus,
                Some('*') => current.operation = Operation::Multiply,
                _ => {}
            }
        }
    }

    if !current.numbers.is_empty() {
        problems.push(current);
    }

    let total_sum: i64 = problems.iter().map(|p| p.solve()).sum();
    println!("Total Sum: {}", total_sum);
}
