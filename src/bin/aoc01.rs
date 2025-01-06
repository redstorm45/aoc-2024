use std::fs;
use std::env;
use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let iter_pairs = contents
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|line| line.split("   ")
                              .map(|v| v.parse::<i32>().unwrap())
                              .collect_tuple::<(i32,i32)>()
                              .unwrap());
    let iter_pairs2 = iter_pairs.clone();
    let mut col1: Vec<i32> = iter_pairs.map(|(a, _)| a).collect();
    let mut col2: Vec<i32> = iter_pairs2.map(|(_, b)| b).collect();

    let dist2: i32 = col1.iter()
                         .map(|a| a * col2.iter().filter(|&x| x==a).count() as i32)
                         .sum();

    col1.sort();
    col2.sort();

    let mut dist = 0;
    for (a,b) in col1.iter().zip(col2.iter()) {
        dist += (a-b).abs();
    }

    println!("Result: {}", dist);
    println!("Result2: {}", dist2);
}
