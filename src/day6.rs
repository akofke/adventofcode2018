use itertools::Itertools;
use ndarray::prelude::*;

#[derive(Debug, Eq, PartialEq)]
struct Point(i32, i32);

fn taxicab_distance(first: &Point, second: &Point) -> i32 {
    (first.0 - second.0).abs() + (first.1 - second.1).abs()
}

fn area_of_point(point: &Point, others: &Vec<Point>, max: (i32, i32)) -> i32 {
    let (max_x, max_y) = max;
    let closest_points = (0..=max_x).cartesian_product((0..=max_y))
        .map(|p| Point(p.0, p.1))
        .filter(|p| {
            let dist = taxicab_distance(point, p);
            others.iter()
                .filter(|&p2| p2 != point)
                .all(|p2| { taxicab_distance(p2, p) > dist})
        });

    let mut count = 0;
    for p in closest_points {
        match p {
            Point(0, _) | Point(_, 0) => return 0,
            Point(x, _) if x == max_x => return 0,
            Point(_, y) if y == max_y => return 0,
            _ => count += 1
        }
    }
    count
}

#[aoc(day6, part1)]
pub fn day6(input: &str) -> i32 {
    let coords: Vec<_> = input.lines()
        .map(|line| {
            let split: Vec<&str> = line.split(", ").collect();
            match &split[..] {
                [a, b] => Point(a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()),
                _ => unreachable!()
            }
        }).collect();

    let (max_x, max_y) = coords.iter().fold((0, 0), |acc, coord| {
        (acc.0.max(coord.0), acc.1.max(coord.1))
    });
    println!("{:?}", (max_x, max_y));

    coords.iter()
        .map(|p| {
            area_of_point(p, &coords, (max_x, max_y))
        })
        .max().unwrap()
}