use tools::read_input_file;

fn main() {
    let input = read_input_file();
    let lines: Vec<&str> = input.lines().collect();
    println!("{:#?}", lines);
}
