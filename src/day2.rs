extern crate itertools;

use self::itertools::Itertools;
use std::collections::HashMap;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let (twos, threes) = input.lines().fold((0, 0), |acc, line| {
        let mut char_counts: HashMap<char, i32> = HashMap::new();
        for c in line.chars() {
            let counter = char_counts.entry(c).or_insert(0);
            *counter += 1;
        }

        let mut t = (0, 0);
        for val in char_counts.values() {
            match val {
                2 => t.0 = 1,
                3 => t.1 = 1,
                _ => (),
            }
        }
        return (acc.0 + t.0, acc.1 + t.1);
    });
    return twos * threes;
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> String {
    let (first, second): (&str, &str) = input
        .lines()
        .cartesian_product(input.lines())
        .find(|(id1, id2)| differs_by_one(id1, id2))
        .unwrap();

    println!("{}, {}", first, second);
    return first
        .chars()
        .zip_eq(second.chars())
        .filter(|(c1, c2)| *c1 == *c2)
        .map(|t| t.1)
        .collect();
}

fn differs_by_one(s1: &str, s2: &str) -> bool {
    let mut same = false;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        match (same, c1 == c2) {
            (false, false) => same = true,
            (true, false) => return false,
            _ => (),
        }
    }
    return same;
}
