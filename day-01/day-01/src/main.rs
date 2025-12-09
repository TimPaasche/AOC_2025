use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

#[derive(PartialEq, Debug)]
enum dir{
    left,
    right,
}



fn main() {
    let mut zero_counter: u32 = 0;
    let mut dial = 50;
    let path = env::current_dir().unwrap().to_str().unwrap().to_string();
    println!("Dir is {}", path);
    let file = File::open(format!("{}\\src\\input.txt", path)).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>().unwrap();
    for line in lines {
        let s = line.trim_start_matches('\u{FEFF}');
        if s.is_empty() { continue; }      // skip blank lines
        if s.len() < 2 { continue; }       // skip malformed lines like "R" or "L"

        let direction = if s.starts_with('R') { dir::right } else { dir::left };
        let steps_str = match s.get(1..) { Some(t) => t.trim(), None => continue };
        let steps = match steps_str.parse::<i32>() {
            Ok(n) => n,
            Err(_) => continue,                               // skip malformed numbers
        };
        dial = rotate_dial(dial, direction, steps, &mut zero_counter);
    }
    println!("counter is {}", zero_counter);
}

fn rotate_dial(start_pos: i32, direction: dir, steps: i32, counter: &mut u32) -> i32 {
    match direction{
        dir::left => {
            let mut pos = start_pos - (steps % 100);
            if pos < 0 { pos += 100}
            if pos == 0 { *counter += 1}
            return pos;
        }
        dir::right => {
            let mut pos = start_pos + (steps % 100);
            if pos > 99 { pos -= 100}
            if pos == 0 { *counter += 1}
            return pos;
        }
    }
}
