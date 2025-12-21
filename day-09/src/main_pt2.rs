use std::collections::{HashMap, VecDeque};
use tools::{read_input_file, write_result_file};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

pub(crate) fn main_pt2() {
    let input = read_input_file();

    let red: Vec<Point> = input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect();

    let n = red.len();

    /* ---------------- coordinate compression ---------------- */

    let mut xs = Vec::new();
    let mut ys = Vec::new();

    for p in &red {
        for d in -1..=1 {
            xs.push(p.x + d);
            ys.push(p.y + d);
        }
    }

    xs.sort();
    xs.dedup();
    ys.sort();
    ys.dedup();

    let x_id: HashMap<i64, usize> =
        xs.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let y_id: HashMap<i64, usize> =
        ys.iter().enumerate().map(|(i, &v)| (v, i)).collect();

    let width = xs.len();
    let height = ys.len();

    /* ---------------- build CELL grid ---------------- */
    let mut grid = vec![vec![0i8; width]; height];
    for x_index in 0..width {
        for y_index in 0..height {
            let y_index_int = y_index as i64;
            let x_index_int = x_index as i64;
            if xs.contains(&x_index_int) && ys.contains(&y_index_int) {
                grid[y_index][x_index] = 1;
            }
        }
    }
    let grid_string = grid_to_string(&grid);
    write_result_file(&grid_string);
}


fn grid_to_string(grid: &Vec<Vec<i8>>) -> String {
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