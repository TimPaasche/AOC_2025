use std::{env, fs, path};

fn pick_k_digits(digits: &[u32], k: usize) -> Vec<u32> {
    let mut stack: Vec<u32> = Vec::new();
    let mut to_remove = digits.len() - k;

    for &d in digits {
        while let Some(&last) = stack.last() {
            if d > last && to_remove > 0 {
                stack.pop();
                to_remove -= 1;
            } else {
                break;
            }
        }
        stack.push(d);
    }

    stack[..k].to_vec()
}

fn main() {
    let file = read_input_file();
    let mut total_sum: u64 = 0;

    for line in file.lines() {
        let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

        let best12 = pick_k_digits(&digits, 12);

        let mut value: u64 = 0;
        for d in &best12 {
            value = value * 10 + (*d as u64);
        }

        println!("Bank {} → {}", line, value);
        total_sum += value;
    }

    println!("Total output joltage: {}", total_sum);
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
