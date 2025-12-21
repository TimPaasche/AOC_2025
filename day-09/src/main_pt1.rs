use tools::read_input_file;

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: u64,
    y: u64,
}

pub(crate) fn main_pt1() {
    let input = read_input_file();
    let lines = input.lines().collect::<Vec<&str>>();
    let points: Vec<Point> = lines
        .iter()
        .map(|line| {
            let (x_str, y_str) = line.split_once(',').unwrap();
            let x = x_str.parse::<u64>().unwrap();
            let y = y_str.parse::<u64>().unwrap();
            Point { x, y }
        })
        .collect();

    println!("{:#?}", points);

    let max_area = max_rectangle_area(&points).unwrap();

    println!("max area: {}", max_area);
}

fn max_rectangle_area(points_vec: &Vec<Point>) -> Option<u64> {
    let mut max = 0;

    for i in 0..points_vec.len() {
        for j in (i + 1)..points_vec.len() {
            max = max.max(calc_rectangle_area(&points_vec[i], &points_vec[j]));
        }
    }
    (points_vec.len() >= 2).then_some(max)
}

fn calc_rectangle_area(p1: &Point, p2: &Point) -> u64 {
    let width = p1.x.abs_diff(p2.x) + 1;
    let height = p1.y.abs_diff(p2.y) + 1;
    width * height
}