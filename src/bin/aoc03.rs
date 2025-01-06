use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let re_instruction = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))").unwrap();

    let mut total = 0;
    let mut total2 = 0;
    let mut active = true;
    for inst in re_instruction.find_iter(&contents) {
        if inst.as_str() == "do()" {
            active = true;
        } else if inst.as_str() == "don't()" {
            active = false;
        } else {
            let (_,[a, b]) = re_mul.captures(inst.as_str()).unwrap().extract();
            let mul_result = a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
            total += mul_result;
            if active {
                total2 += mul_result;
            }
        }
    }

    println!("Result: {}", total);
    println!("Result2: {}", total2);
}
