use itertools::Itertools;

const LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyz";

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

//fn apply_reactions_iter<I, O>(compound: I) -> O
//    where I: Itertools<Item=u8>,
//          O: Itertools<Item=u8>
//{
//    let mut found_reaction = false;
//    let new_compound = compound.coalesce(|a, b| {
//        if diff_ascii_case(a, b) {
//            found_reaction = true;
//            Ok(0)
//        } else {
//            Err((a, b))
//        }
//    }).filter(|b| { *b != 0 });
//
//    match found_reaction {
//        true => apply_reactions_iter(new_compound),
//        false => new_compound
//    }
//}

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