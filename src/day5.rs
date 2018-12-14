use itertools::Itertools;

const LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyz";

fn diff_ascii_case(a: u8, b: u8) -> bool {
    (a as i8 - b as i8).abs() == 32i8
}

fn make_regex() -> regex::Regex {
    let re_str = LETTERS.chars().map(|c| {
        format!(r"({}{})|({}{})", c, c.to_ascii_uppercase(), c.to_ascii_uppercase(), c)
    }).join("|");
    regex::Regex::new(&re_str).unwrap()
}


fn apply_reactions_regex(re: &regex::Regex, compound: &str) -> (bool, String) {
    let has_reaction = re.is_match(compound);
    (has_reaction, re.replace_all(compound, "").to_string())
}

fn fully_react_regex(compound: &str) -> String {
    let re = make_regex();
    let mut compound = compound.to_string();
    while let (true, new_compound) = apply_reactions_regex(&re, &compound) {
        compound = new_compound
    }
    compound
}

fn apply_reactions(compound: &str) -> (bool, String) {
    let mut found_reaction = false;
    let new_compound = compound.bytes().coalesce(|a, b| {
        if diff_ascii_case(a, b) {
            found_reaction = true;
            Ok(0)
        } else {
            Err((a, b))
        }
    }).filter(|b| { *b != 0 }).collect();
    (found_reaction, String::from_utf8(new_compound).unwrap())
}

fn fully_react_iterative(compound: &str) -> String {
    let mut compound = compound.to_string();
    while let (true, new_compound) = apply_reactions(&compound) {
        compound = new_compound
    }
    compound.to_string()
}

fn fully_react(compound: &str) -> String {
    let mut reacted = Vec::new();
    for b in compound.bytes() {
        match reacted.pop() {
            None => {
                reacted.push(b);
            },
            Some(a) => {
                if !diff_ascii_case(a, b) {
                    reacted.push(a);
                    reacted.push(b);
                }
            }
        }
    }
    unsafe {
        String::from_utf8_unchecked(reacted)
    }
}

#[aoc(day5, part1, regex)]
pub fn part1_regex(input: &str) -> usize {
    let reacted = fully_react_regex(input);
    reacted.len()
}

#[aoc(day5, part1, iterative)]
pub fn part1_iter(input: &str) -> usize {
    let reacted = fully_react_iterative(input);
    reacted.len()
}

#[aoc(day5, part1, fast)]
pub fn part1(input: &str) -> usize {
    let reacted = fully_react(input);
    reacted.len()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    LETTERS.bytes()
        .map(|remove_unit| {
            let bytes: Vec<u8> = input.bytes()
                .filter(|c| { !c.eq_ignore_ascii_case(&remove_unit) })
                .collect();
            String::from_utf8(bytes).unwrap()
        })
        .map(|filtered| {
            fully_react(&filtered).len()
        })
        .min().unwrap()
}