use std::fs;
use std::env;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE_MOVE: Regex = Regex::new(r"Button (?:A|B): X\+(\d+), Y\+(\d+)").unwrap();
    static ref RE_PRIZE: Regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let machines: Vec<ClawMachine> = contents.split("\n\n")
        .map(parse_claw_machine)
        .collect();
    let cost = machines.iter()
            .filter_map(get_inverse)
            .filter(|(a,b)| *a <= 100 && *b <= 100)
            .map(|(a,b)| 3*a+b)
            .sum::<usize>();
    let cost_big = machines.iter()
            .map(|m| m.bigprize())
            .filter_map(|m| get_inverse(&m))
            .map(|(a,b)| 3*a+b)
            .sum::<usize>();

    println!("Result: {}", cost);
    println!("Result2: {}", cost_big);
}

#[derive(Debug)]
struct ClawMachine {
    movea: (usize, usize),
    moveb: (usize,usize),
    prize: (usize,usize)
}

impl ClawMachine {
    fn bigprize(&self) -> ClawMachine {
        ClawMachine {
            movea: self.movea,
            moveb: self.moveb,
            prize: (self.prize.0 + 10_000_000_000_000, self.prize.1 + 10_000_000_000_000),
        }
    }
}

fn parse_claw_machine(s: &str) -> ClawMachine {
    let mut it = s.split('\n');
    let linea = it.next().unwrap();
    let lineb = it.next().unwrap();
    let linep = it.next().unwrap();

    let (_, coords_a): (_, [&str; 2]) = RE_MOVE.captures(linea).unwrap().extract();
    let (_, coords_b): (_, [&str; 2]) = RE_MOVE.captures(lineb).unwrap().extract();
    let (_, coords_p): (_, [&str; 2]) = RE_PRIZE.captures(linep).unwrap().extract();

    ClawMachine{
        movea: (coords_a[0].parse().unwrap(), coords_a[1].parse().unwrap()),
        moveb: (coords_b[0].parse().unwrap(), coords_b[1].parse().unwrap()),
        prize: (coords_p[0].parse().unwrap(), coords_p[1].parse().unwrap()),
    }
}

fn get_inverse(m: &ClawMachine) -> Option<(usize,usize)> {
    // try to use a matrix inverse to get integers satisfying the conditions
    let determinant = ((m.movea.0*m.moveb.1) as isize) - ((m.movea.1*m.moveb.0) as isize);
    if determinant == 0 {
        // both moves aligned
        panic!("Aligned moves");
        None
    } else {
        let rowa = ((m.prize.0*m.moveb.1) as isize) - ((m.prize.1*m.moveb.0) as isize);
        let rowb = ((m.prize.1*m.movea.0) as isize) - ((m.prize.0*m.movea.1) as isize);
    
        if rowa%determinant == 0 && rowb%determinant == 0 {
            let counta = rowa/determinant;
            let countb = rowb/determinant;
            if counta >= 0 && countb >= 0 {
                return Some((counta as usize, countb as usize));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let parsed = parse_claw_machine("Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400");
        println!("{:?}", parsed);
        assert_eq!(get_inverse(&parsed), Some((80,40)));
    }
}
