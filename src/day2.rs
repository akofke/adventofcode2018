use std::collections::HashMap;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let (twos, threes) = input.lines()
        .fold((0, 0), |acc, line| {
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
                    _ => ()
                }
            }
            return (acc.0 + t.0, acc.1 + t.1);
        });
    return twos * threes;
}