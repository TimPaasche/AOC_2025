use std::collections::{HashSet, VecDeque};

use tools::read_input_file;

#[derive(Debug)]
struct Instruction {
    goal: u16,
    buttons: Vec<u16>,
    // rest: String,
}

fn main() {
    let input: String = read_input_file();
    let lines: Vec<&str> = input.lines().collect();
    let mut count: u32 = 0;
    for line in lines {
        count += minimal_moves_to_resolve(line);
    }
    println!("count: {:?}", count);
}

fn minimal_moves_to_resolve(input_line: &str) -> u32 {
    let instructions = parse_line(input_line);
    breadth_first_search(instructions)
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
    // let rest = parts.last().unwrap();
    Instruction {
        goal,
        buttons,
        // rest: rest.to_string(),
    }
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

fn breadth_first_search(content: Instruction) -> u32 {
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
