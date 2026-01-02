use good_lp::{variable, ProblemVariables, default_solver, Expression, constraint, SolverModel, Solution};

#[derive(Debug)]
struct Instruction {
    buttons: Vec<Vec<u16>>,
    joltage_regulations: Vec<u16>,
}

fn main() {
    let input = tools::read_input_file();
    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();
    let instructions: Vec<Instruction> = lines.iter().map(|line| parse_instruction(line)).collect();
    let mut counter: u64 = 0;
    for instruction in &instructions {
        counter += solve_button_presses_ilp(instruction);
    }
    println!("Counter of clicks: {}", counter);
}

fn parse_instruction(line: &str) -> Instruction {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let joltage_regulations: Vec<u16> = parts
        .last()
        .unwrap()
        .trim_matches(|c| c == '{' || c == '}')
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let vec_size = joltage_regulations.len();
    let buttons_count = parts.len() - 2;
    let mut buttons: Vec<Vec<u16>> = vec![vec![0; vec_size]; buttons_count];
    for (idx, part) in parts.iter().skip(1).enumerate() {
        if part.starts_with("{") {
            break;
        }

        let indices: Vec<u16> = part
            .trim_matches(|c| c == '(' || c == ')')
            .split(',')
            .filter_map(|s| s.parse::<u16>().ok())
            .collect();

        for &index in &indices {
            buttons[idx][index as usize] = 1;
        }
    }
    Instruction {
        buttons,
        joltage_regulations,
    }
}

fn solve_button_presses_ilp(instruction: &Instruction) -> u64 {
    let num_buttons = instruction.buttons.len();
    let num_joltages = instruction.joltage_regulations.len();

    // Create problem variables and add one integer variable per button (>= 0)
    let mut vars = ProblemVariables::new();
    let buttons_vars: Vec<_> = (0..num_buttons)
        .map(|_| vars.add(variable().integer().min(0)))
        .collect();

    // Objective: minimize the total number of button presses
    let objective: Expression = buttons_vars.iter().cloned().sum();

    // Build the solver model
    let mut model = vars.minimise(objective.clone()).using(default_solver);

    // Add constraints: for each joltage index, sum(button_i * covers_ij) == regulation_j
    for j in 0..num_joltages {
        let sum_expr: Expression = (0..num_buttons)
            .map(|i| buttons_vars[i] * instruction.buttons[i][j] as f64)
            .sum();
        model = model.with(constraint!(sum_expr == instruction.joltage_regulations[j] as f64));
    }

    let solution = model.solve().unwrap();

    // Evaluate the objective using the solution
    solution.eval(&objective).round() as u64
}
