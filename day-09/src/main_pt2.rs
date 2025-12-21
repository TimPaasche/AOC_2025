use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};
use tools::{read_input_file, write_result_file};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

pub(crate) fn main_pt2() {
    let input_content = read_input_file();

    let vertices: Vec<Point> = input_content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (x_str, y_str) = line.split_once(',').unwrap();
            Point {
                x: x_str.trim().parse().unwrap(),
                y: y_str.trim().parse().unwrap(),
            }
        })
        .collect();

    let mut unique_x_coordinates = Vec::new();
    let mut unique_y_coordinates = Vec::new();

    for point in &vertices {
        for offset in -1..=1 {
            unique_x_coordinates.push(point.x + offset);
            unique_y_coordinates.push(point.y + offset);
        }
    }

    unique_x_coordinates.sort();
    unique_x_coordinates.dedup();
    unique_y_coordinates.sort();
    unique_y_coordinates.dedup();

    let x_coordinate_to_index: HashMap<i64, usize> = unique_x_coordinates
        .iter()
        .enumerate()
        .map(|(index, &value)| (value, index))
        .collect();

    let y_coordinate_to_index: HashMap<i64, usize> = unique_y_coordinates
        .iter()
        .enumerate()
        .map(|(index, &value)| (value, index))
        .collect();

    let grid_width = unique_x_coordinates.len();
    let grid_height = unique_y_coordinates.len();

    let mut grid = vec![vec![0i8; grid_width]; grid_height];

    for point in &vertices {
        let x_index = x_coordinate_to_index[&point.x];
        let y_index = y_coordinate_to_index[&point.y];
        grid[y_index][x_index] = 1;
    }

    connect_grid_vertices(&mut grid);
    fill_grid_interior(&mut grid);
    let max_rect_area = max_rectangle_area(&vertices, &grid, &x_coordinate_to_index, &y_coordinate_to_index).unwrap();
    println!("max area: {}", max_rect_area);
    let result_string = convert_grid_to_string(&grid);
    write_result_file(&result_string);
}

fn connect_grid_vertices(grid: &mut Vec<Vec<i8>>) {
    let height = grid.len();
    let width = grid[0].len();

    for y_index in 0..height {
        let mut last_vertex_x = None;
        for x_index in 0..width {
            if grid[y_index][x_index] == 1 {
                if let Some(previous_x) = last_vertex_x {
                    for fill_x in (previous_x + 1)..x_index {
                        grid[y_index][fill_x] = 1;
                    }
                }
                last_vertex_x = Some(x_index);
            }
        }
    }

    for x_index in 0..width {
        let mut last_vertex_y = None;
        for y_index in 0..height {
            if grid[y_index][x_index] == 1 {
                if let Some(previous_y) = last_vertex_y {
                    for fill_y in (previous_y + 1)..y_index {
                        grid[fill_y][x_index] = 1;
                    }
                }
                last_vertex_y = Some(y_index);
            }
        }
    }
}

fn fill_grid_interior(grid: &mut Vec<Vec<i8>>) {
    let height = grid.len();
    let width = grid[0].len();
    let mut flood_queue = VecDeque::new();

    for x_index in 0..width {
        flood_queue.push_back((x_index, 0));
        flood_queue.push_back((x_index, height - 1));
    }
    for y_index in 0..height {
        flood_queue.push_back((0, y_index));
        flood_queue.push_back((width - 1, y_index));
    }

    while let Some((current_x, current_y)) = flood_queue.pop_front() {
        if grid[current_y][current_x] == 0 {
            grid[current_y][current_x] = -1;

            let neighbor_offsets = [(0, 1), (0, -1), (1, 0), (-1, 0)];
            for (delta_x, delta_y) in neighbor_offsets {
                let neighbor_x = current_x as i32 + delta_x;
                let neighbor_y = current_y as i32 + delta_y;

                if neighbor_x >= 0
                    && neighbor_x < width as i32
                    && neighbor_y >= 0
                    && neighbor_y < height as i32
                {
                    flood_queue.push_back((neighbor_x as usize, neighbor_y as usize));
                }
            }
        }
    }

    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            if *cell == -1 {
                *cell = 0;
            } else if *cell == 0 {
                *cell = 2;
            }
        }
    }
}

fn convert_grid_to_string(grid: &Vec<Vec<i8>>) -> String {
    grid.iter()
        .map(|row| {
            row.iter()
                .map(|cell| cell.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn validate_if_rect_is_inside(
    grid: &Vec<Vec<i8>>,
    boundarybox_point1: &Point,
    boundarybox_point2: &Point,
    x_coordinate_to_index: &HashMap<i64, usize>,
    y_coordinate_to_index: &HashMap<i64, usize>,
) -> bool {
    let start_x_index = *x_coordinate_to_index.get(&boundarybox_point1.x).unwrap();
    let end_x_index = *x_coordinate_to_index.get(&boundarybox_point2.x).unwrap();
    let start_y_index = *y_coordinate_to_index.get(&boundarybox_point1.y).unwrap();
    let end_y_index = *y_coordinate_to_index.get(&boundarybox_point2.y).unwrap();

    let min_x = start_x_index.min(end_x_index);
    let max_x = start_x_index.max(end_x_index);
    let min_y = start_y_index.min(end_y_index);
    let max_y = start_y_index.max(end_y_index);

    for current_y in min_y..=max_y {
        for current_x in min_x..=max_x {
            if grid[current_y][current_x] == 0 {
                return false;
            }
        }
    }
    true
}

fn max_rectangle_area(
    points_vec: &Vec<Point>,
    grid: &Vec<Vec<i8>>,
    x_coordinate_to_index: &HashMap<i64, usize>,
    y_coordinate_to_index: &HashMap<i64, usize>,
) -> Option<u64> {
    let mut max = 0;

    for i in 0..points_vec.len() {
        for j in (i + 1)..points_vec.len() {
            if !validate_if_rect_is_inside(
                grid, &points_vec[i], &points_vec[j],
                x_coordinate_to_index,
                y_coordinate_to_index,
            ) {
                continue;
            }

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
