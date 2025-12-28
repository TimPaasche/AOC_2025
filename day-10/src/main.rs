use tools::read_input_file;

fn main() {
    let input: String = read_input_file();
    let lines: Vec<&str> = input.lines().collect();
    // println!("{:#?}", lines);
    let first_example: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
    let first_desired_result = first_example.split_once(' ').unwrap().0;
    let first_btns = first_example.split_once(' ').unwrap().1;
    println!("{:?}", first_desired_result);
    println!("{:?}", first_btns);
    let secand_example: &str = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
    // assert_eq!(, erwarteter_wert);
    // assert_eq!(link_wert, erwarteter_wert);
}
