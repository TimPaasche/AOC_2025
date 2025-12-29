use tools::read_input_file;

fn main() {
    let input: String = read_input_file();
    let lines: Vec<&str> = input.lines().collect();
    // println!("{:#?}", lines);
    let first_example: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
    let secand_example: &str = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
    let result = minimal_moves_to_resolve(first_example);
    // assert_eq!(, erwarteter_wert);
    // assert_eq!(link_wert, erwarteter_wert);
}

fn minimal_moves_to_resolve(input_line: &str) -> u32 {
    let input_line_split: (&str, &str) = input_line.split_once(' ').unwrap();
    let expected_rusult: Vec<bool> = input_line_split
        .0
        .trim_end_matches(|c| c == '[' || c == ']')
        .chars()
        .map(|c| c == '#')
        .collect();
    let result: Box<[bool]> = vec![false; expected_result.len() as usize].into_boxed_slice();
    todo!();
}
