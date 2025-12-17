use std::str::Chars;
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

fn main() {
    let input = read_input_file();

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    if grid.is_empty() {
        return;
    }
    let mut mathProblems: Vec<MathProblem> = vec![];
    let mut temp_mathProblem: MathProblem = MathProblem {
        numbers: Vec::new(),
        operation: Operation::None,
    };
    for index in 0..grid[0].len() {
        if (!grid
            .get(0)
            .map_or(false, |row| row[index].is_alphanumeric())
            && !grid
                .get(1)
                .map_or(false, |row| row[index].is_alphanumeric())
            && !grid
                .get(2)
                .map_or(false, |row| row[index].is_alphanumeric())
            && !grid
                .get(3)
                .map_or(false, |row| row[index].is_alphanumeric()))
        {
            mathProblems.push(temp_mathProblem);
            temp_mathProblem = MathProblem {
                numbers: Vec::new(),
                operation: Operation::None,
            };
            continue;
        }
        let mut number_as_string = String::new();

        // Safely grab the character at 'index' for each of the first three rows
        for row_idx in 0..=3 {
            if let Some(row) = grid.get(row_idx) {
                if let Some(&character) = row.get(index) {
                    number_as_string.push(character);
                }
            }
        }
        temp_mathProblem.numbers.push(number_as_string.trim().parse::<i64>().unwrap());
        if index == grid[0].len().saturating_sub(1) {
            mathProblems.push(temp_mathProblem.clone());
            continue;
        }

        if grid.get(4).map_or(false, |row| row[index] == '+') {
            temp_mathProblem.operation = Operation::Plus;
        } else if grid.get(4).map_or(false, |row| row[index] == '*') {
            temp_mathProblem.operation = Operation::Multiply;
        }
    }
    let sum = mathProblems.iter().map(|mp| mp.solve()).sum::<i64>();

    println!("Sum: {}", sum);
}

impl MathProblem {
    fn solve(&self) -> i64 {
        match self.operation {
            Operation::Plus => self.numbers.iter().sum(),
            Operation::Multiply => self.numbers.iter().product(),
            Operation::None => 0,
        }
    }
}
