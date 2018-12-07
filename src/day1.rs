use std::collections::HashSet;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    return input
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .fold(0, |acc, x| acc + x);
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut set = HashSet::new();
    input
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .cycle()
        .scan(0, |acc, x| {
            *acc = *acc + x;
            Some(*acc)
        })
        //        .inspect(|x| println!("{}", x))
        .find(|x| !set.insert(*x))
        .unwrap()
}
