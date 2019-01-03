extern crate ndarray;

use ndarray::prelude::*;
use ndarray::{s};

pub fn max_cell(grid: &Array2<i32>, cell_size: usize) -> (usize, usize, i32) {
    let cell = grid.indexed_iter()
        .map(|((x, y), _)| {
            if x > grid.cols() - cell_size || y > grid.rows() - cell_size {
                return (x, y, 0);
            }

            let tot_power = grid.slice(s![x..x+cell_size, y..y+cell_size]).sum();
            (x, y, tot_power)
        })
        .max_by_key(|&(_, _, power)| power).unwrap();
    

    (cell.0 + 1, cell.1 + 1, cell.2)
}

fn get_power(x: usize, y: usize, serial_num: i32) -> i32 {
        let rack_id = x as i32 + 1 + 10;
        let mut power: i32 = rack_id * (y as i32 + 1);
        power += serial_num;
        power *= rack_id;
        power = power / 100 % 10;
        power - 5
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> String {
    let serial_num: i32 = input.parse().unwrap();
    let grid = Array::from_shape_fn((300, 300), |(x, y)| {
        get_power(x, y, serial_num)
    });

    // println!("{:?}", grid);

    format!("{:?}", max_cell(&grid, 3))
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> String {
    // TODO: improve performance (dynamic programming?)
    let serial_num: i32 = input.parse().unwrap();
    let grid = Array::from_shape_fn((300, 300), |(x, y)| {
        get_power(x, y, serial_num)
    });

    let cell = (1..301).map(|size| {
        (max_cell(&grid, size), size)
    }).max_by_key(|&(cell, s)| cell.2);

    format!("{:?}", cell)
}