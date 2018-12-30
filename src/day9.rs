use regex::Regex;

struct Spec {
    players: u32,
    last_marble: u32,
}

fn parse_input(input: &str) -> Spec {
    let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    let caps = re.captures(input).unwrap();
    Spec {
        players: caps.get(1).unwrap().as_str().parse::<u32>().unwrap(),
        last_marble: caps.get(2).unwrap().as_str().parse().unwrap()
    }
}

fn simulate_game(spec: Spec) -> u32 {
    unimplemented!()
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> u32 {
    let spec = parse_input(input);

    0
}