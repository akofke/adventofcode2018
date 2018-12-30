use ndarray::prelude::*;
use regex::Regex;
use bmp::{Image, px, Pixel};


fn parse_input(input: &str) -> (Array2<i32>, Array2<i32>) {
    let num = r"\s?(-?\d+)";
    let re = Regex::new(&format!(r"position=<{0}, {0}> velocity=<{0}, {0}>", num)).unwrap();
    let count = input.lines().count();
    let mut positions = ndarray::Array::zeros((count, 2));
    let mut velocities = ndarray::Array::zeros((count, 2));
    for (i, caps) in re.captures_iter(input).enumerate() {
        let (xx, xy, vx, vy) = (&caps[1], &caps[2], &caps[3], &caps[4]);
        positions[[i, 0]] = xx.parse::<i32>().unwrap();
        positions[[i, 1]] = xy.parse::<i32>().unwrap();
        velocities[[i, 0]] = vx.parse::<i32>().unwrap();
        velocities[[i, 1]] = vy.parse::<i32>().unwrap();
    }

    (positions, velocities)
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> String {
    let (mut pos, vel) = parse_input(input);
    // println!("{:?}", pos);
    let mut min_x_seen = 999999;
    let mut time = 0;

    loop {
        let &max_x = pos.column(0).iter().max().unwrap();
        let &min_x = pos.column(0).iter().min().unwrap();
        let &max_y = pos.column(1).iter().max().unwrap();
        let &min_y = pos.column(1).iter().min().unwrap();
        if max_x < 1000 && max_x < min_x_seen {
            min_x_seen = max_x;
            println!("{} {} {} {}", min_x, max_x, min_y, max_y);
            println!("{}", time);
            let mut img = Image::new((max_x + min_x.abs() + 1) as u32, (max_y + min_y.abs() + 1) as u32);
            pos.outer_iter().for_each(|r| { 
                img.set_pixel(
                    (r[0] + min_x.abs()) as u32,
                    (r[1] + min_y.abs()) as u32,
                    px!(255, 255, 255))
            });
            img.save("test.bmp").expect("wat");
            // panic!();
        }

        pos += &vel;
        time += 1;
    }
    // let mut img = Image.new()
    unimplemented!()
}