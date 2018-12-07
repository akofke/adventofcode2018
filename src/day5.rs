use itertools::Itertools;

fn diff_ascii_case(a: u8, b: u8) -> bool {
    (a as i8 - b as i8).abs() == 32i8
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

fn fully_react(compound: &str) -> String {
    let mut compound = compound.to_string();
    while let (true, new_compound) = apply_reactions(&compound) {
        compound = new_compound
    }
    compound.to_string()
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> i32 {
    let reacted = fully_react(input);

    reacted.len() as i32
}