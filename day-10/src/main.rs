use std::collections::{HashSet, VecDeque};

use tools::read_input_file;

#[derive(Debug)]
struct Instruction {
    goal: u16,
    buttons: Vec<u16>,
    joltage_values: Vec<u16>,
}

fn main() {
    let input: String = read_input_file();
    let lines: Vec<&str> = input.lines().collect();
    let mut count_startup: u32 = 0;
    let mut count_joltage_value: u32 = 0;
    for line in lines {
        count_startup += minimal_moves_to_resolve_startup(line);
        count_joltage_value += minimal_moves_to_resolve_joltage_value(line);
    }
    println!("count: {:?}", count_startup);
}

fn minimal_moves_to_resolve_startup(input_line: &str) -> u32 {
    let instructions = parse_line(input_line);
    bfs_startup(instructions)
}

fn minimal_moves_to_resolve_joltage_value(input_line: &str) -> u32 {
    let instruction = parse_line(input_line);
    bfs_joltage_value(instruction)
}

fn parse_line(line: &str) -> Instruction {
    let parts: Vec<&str> = line.split_whitespace().collect();

    //parsing if the first part (goal)
    let goal_str = parts[0].trim_matches(|c| c == '[' || c == ']');
    let mut goal: u16 = 0;
    for (ii, cc) in goal_str.chars().enumerate() {
        if cc == '#' {
            goal |= 1 << ii;
        }
    }

    //parsing the buttons
    let mut buttons = Vec::new();
    for part in parts.iter().skip(1) {
        if part.starts_with('{') {
            break;
        }
        buttons.push(parse_button_str(part));
    }

    //parsing the rest
    let part_last = parts.last().unwrap();
    let joltage_values: Vec<u16> = parse_joltage_values(part_last);
    Instruction {
        goal,
        buttons,
        joltage_values,
    }
}
fn parse_joltage_values(joltage_str: &str) -> Vec<u16> {
    let mut rtn_joltage_values = Vec::new();
    let joltige_str_trimmed: &str = joltage_str.trim_matches(|c| c == '{' || c == '}');
    joltige_str_trimmed.split(',').for_each(|joltage_value| {
        if let Ok(val) = joltage_value.parse::<u16>() {
            rtn_joltage_values.push(val);
        }
    });
    rtn_joltage_values
}

fn parse_button_str(button_str: &str) -> u16 {
    let content = button_str.trim_matches(|c| c == '(' || c == ')');
    let mut btn_mask = 0;

    content.split(',').for_each(|num_str| {
        if let Ok(idx) = num_str.parse::<u16>() {
            btn_mask |= 1 << idx;
        }
    });
    btn_mask
}

fn bfs_startup(content: Instruction) -> u32 {
    // Queue stores: (current_light_state, number_of_presses_so_far)
    let mut queue: VecDeque<(u16, u32)> = VecDeque::new();
    let mut visited: HashSet<u16> = HashSet::new();

    // We start with all lights OFF (state 0) and 0 presses.
    queue.push_back((0, 0));
    visited.insert(0);

    // 3. Run BFS
    while let Some((current_state, presses)) = queue.pop_front() {
        // Did we match the pattern in the manual?
        if current_state == content.goal {
            return presses;
        }

        // Try pressing every available button
        for button_mask in &content.buttons {
            // XOR toggles the bits where the button has a 1
            let next_state = current_state ^ button_mask;

            // If we haven't seen this light configuration before, add it to the queue
            if visited.insert(next_state) {
                queue.push_back((next_state, presses + 1));
            }
        }
    }

    panic!("No solution found for this machine!");
}

fn bfs_joltage_value(content: Instruction) -> u32 {
    let number_of_joltage_values: usize = content.joltage_values.len();
    // Queue stores: (current_joltage_state, number_of_presses_so_far)
    let mut queue: VecDeque<(Vec<u16>, u32)> = VecDeque::new();
    let mut visited: HashSet<u16> = HashSet::new();

    // We start with all lights OFF (state 0) and 0 presses.
    queue.push_back((vec![0; number_of_joltage_values], 0));
    visited.insert(0);
    todo!()
}
