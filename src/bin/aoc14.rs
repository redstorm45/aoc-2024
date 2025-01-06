use std::fs;
use std::env;
use regex::Regex;
use itertools::Itertools;
use lazy_static::lazy_static;
//use std::{thread, time};

lazy_static! {
    static ref RE_BOT: Regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let robots: Vec<RobotDescription> = contents.split_terminator('\n').map(parse_robot).collect();
    let after_move: Vec<(i32,i32)> = robots.iter().map(|r| simulate_moves(r, 100, 101, 103)).collect();

    let result = safety_factor(&after_move, 101, 103);

    println!("Result {}", result);

    for i in 1..1000000 {
        let moved = robots.iter().map(|r| simulate_moves(r, i, 101, 103)).collect::<Vec<(i32,i32)>>();
        let variance = variances(&moved);
        if variance < 900.0f32 {
            /*
            let pic = _make_picture(&moved);
            println!("{}\nAfter {} moves, variance: {}", pic, i, variance);
            std::thread::sleep(time::Duration::from_millis(500));
            */
            println!("Result2: {}", i);
            break;
        }
    }
}

struct RobotDescription {
    origin: (i32,i32),
    speed: (i32,i32),
}

fn parse_robot(s: &str) -> RobotDescription {
    // p=0,4 v=3,-3
    let (_, groups): (_,[&str;4]) = RE_BOT.captures(s).unwrap().extract();
    RobotDescription{
        origin: (
            groups[0].parse().unwrap(),
            groups[1].parse().unwrap(),
        ), speed: (
            groups[2].parse().unwrap(),
            groups[3].parse().unwrap(),
        )
    }
}

fn simulate_moves(r: &RobotDescription, steps: i32, width: i32, height: i32) -> (i32, i32) {
    let end_x = ((r.origin.0 + steps*r.speed.0)%width + width)%width;
    let end_y = ((r.origin.1 + steps*r.speed.1)%height + height)%height;
    (end_x, end_y)
}

fn safety_factor(bots: &[(i32,i32)], width: i32, height: i32) -> i32 {
    let midx = (width-1)/2;
    let midy = (height-1)/2;
    let quadrants = bots
        .iter()
        .sorted_by_key(|(x,y)| (x.cmp(&midx), y.cmp(&midy)))
        .chunk_by(|(x,y)| (x.cmp(&midx), y.cmp(&midy)));

    let mut result = 1;
    for (key, it) in quadrants.into_iter() {
        if key.0 != std::cmp::Ordering::Equal && key.1 != std::cmp::Ordering::Equal {
            /*
            let in_group: Vec<(i32,i32)> = it.cloned().collect();
            println!("In quadarant {:?} at {:?}", key, in_group);
            result *= in_group.len() as i32;
            */
            result *= it.count() as i32;
        }
    }

    result
}

fn _make_picture(bots: &[(i32,i32)]) -> String {
    let mut res: String = String::new();
    for i in 0..103 {
        let s: String = (0..101)
            .map(|j| bots.contains(&(i,j)))
            .map(|v| if v {'#'} else {'.'})
            .collect();
        res += "\n";
        res += s.as_str();
    }
    res
}

fn variances(bots: &[(i32,i32)]) -> f32 {
    let meanx: f32 = bots.iter().map(|(x,_)| *x as f32).sum::<f32>() / (bots.len() as f32);
    let meany = bots.iter().map(|(_,y)| *y as f32).sum::<f32>() / (bots.len() as f32);
    let varx = bots.iter().map(|(x,_)| (*x as f32-meanx)*(*x as f32-meanx)).sum::<f32>() / (bots.len() as f32);
    let vary = bots.iter().map(|(_,y)| (*y as f32-meany)*(*y as f32-meany)).sum::<f32>() / (bots.len() as f32);
    varx + vary
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_move() {
        assert_eq!(simulate_moves(&RobotDescription{origin:(2,4), speed:(2,-3)}, 5, 11, 7), (1,3));
    }

    #[test]
    fn example_quadrant() {
        let txt = "p=0,4 v=3,-3\np=6,3 v=-1,-3\np=10,3 v=-1,2\np=2,0 v=2,-1\np=0,0 v=1,3\np=3,0 v=-2,-2\np=7,6 v=-1,-3\np=3,0 v=-1,-2\np=9,3 v=2,3\np=7,3 v=-1,2\np=2,4 v=2,-3\np=9,5 v=-3,-3";
        let robots: Vec<RobotDescription> = txt.split('\n').map(parse_robot).collect();
        let after_move: Vec<(i32,i32)> = robots.iter().map(|r| simulate_moves(r, 100, 11, 7)).collect();
        println!("After moves: {:?}", after_move);
        let safety = safety_factor(&after_move, 11, 7);
        assert_eq!(safety, 12);
    }
}