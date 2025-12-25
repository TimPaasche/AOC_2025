use std::cmp::{max, min};
use std::collections::{HashMap};
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

    let mut unique_x: Vec<i64> = vertices.iter().map(|p| p.x).collect();
    let mut unique_y: Vec<i64> = vertices.iter().map(|p| p.y).collect();

    unique_x.sort();
    unique_x.dedup();
    unique_y.sort();
    unique_y.dedup();

    let x_map: HashMap<i64, usize> = unique_x.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let y_map: HashMap<i64, usize> = unique_y.iter().enumerate().map(|(i, &v)| (v, i)).collect();

    let grid_width = unique_x.len().saturating_sub(1);
    let grid_height = unique_y.len().saturating_sub(1);

    let mut invalid_grid = vec![vec![0; grid_width]; grid_height];

    for y in 0..grid_height {
        for x in 0..grid_width {
            let mid_x = (unique_x[x] as f64 + unique_x[x + 1] as f64) / 2.0;
            let mid_y = (unique_y[y] as f64 + unique_y[y + 1] as f64) / 2.0;

            if !is_inside_polygon(mid_x, mid_y, &vertices) {
                invalid_grid[y][x] = 1;
            }
        }
    }

    let mut prefix_sum = vec![vec![0; grid_width + 1]; grid_height + 1];

    for y in 0..grid_height {
        for x in 0..grid_width {
            prefix_sum[y + 1][x + 1] = invalid_grid[y][x]
                + prefix_sum[y][x + 1]
                + prefix_sum[y + 1][x]
                - prefix_sum[y][x];
        }
    }

    let max_area = solve_max_area(&vertices, &x_map, &y_map, &prefix_sum);

    println!("max area: {}", max_area);
    write_result_file(&max_area.to_string());
}

fn is_inside_polygon(test_x: f64, test_y: f64, polygon: &[Point]) -> bool {
    let mut inside = false;
    let n = polygon.len();
    for i in 0..n {
        let j = (i + 1) % n;
        let pi = polygon[i];
        let pj = polygon[j];

        let intersects = ((pi.y as f64) > test_y) != ((pj.y as f64) > test_y)
            && (test_x < (pj.x as f64 - pi.x as f64) * (test_y - pi.y as f64)
            / (pj.y as f64 - pi.y as f64) + pi.x as f64);

        if intersects {
            inside = !inside;
        }
    }
    inside
}

fn solve_max_area(
    vertices: &[Point],
    x_map: &HashMap<i64, usize>,
    y_map: &HashMap<i64, usize>,
    prefix_sum: &Vec<Vec<i32>>,
) -> u64 {
    let mut max_area = 0;
    let n = vertices.len();

    for i in 0..n {
        for j in (i + 1)..n {
            let p1 = vertices[i];
            let p2 = vertices[j];

            let x1_idx = *x_map.get(&p1.x).unwrap();
            let x2_idx = *x_map.get(&p2.x).unwrap();
            let y1_idx = *y_map.get(&p1.y).unwrap();
            let y2_idx = *y_map.get(&p2.y).unwrap();

            let min_x = min(x1_idx, x2_idx);
            let max_x = max(x1_idx, x2_idx);
            let min_y = min(y1_idx, y2_idx);
            let max_y = max(y1_idx, y2_idx);

            if min_x == max_x || min_y == max_y {
                let area = (p1.x.abs_diff(p2.x) + 1) * (p1.y.abs_diff(p2.y) + 1);
                if is_region_valid(min_x, max_x, min_y, max_y, prefix_sum) {
                    max_area = max(max_area, area);
                }
                continue;
            }

            if is_region_valid(min_x, max_x, min_y, max_y, prefix_sum) {
                let area = (p1.x.abs_diff(p2.x) + 1) * (p1.y.abs_diff(p2.y) + 1);
                max_area = max(max_area, area);
            }
        }
    }
    max_area
}

fn is_region_valid(
    x1: usize, x2: usize,
    y1: usize, y2: usize,
    prefix_sum: &Vec<Vec<i32>>
) -> bool {
    if x1 == x2 || y1 == y2 {
        return true;
    }

    let sum = prefix_sum[y2][x2]
        - prefix_sum[y1][x2]
        - prefix_sum[y2][x1]
        + prefix_sum[y1][x1];

    sum == 0
}
