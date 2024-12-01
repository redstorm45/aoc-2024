use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let m: Vec<Vec<i32>> = contents
        .split('\n')
        .filter(|s| s.len()>0)
        .map(|line| line.split("   ")
                              .map(|v| v.parse::<i32>().unwrap())
                              .collect::<Vec<i32>>())
        .collect();
    let col1: Vec<i32> = m.iter().map(|v| *v.first().unwrap()).collect();
    let col2: Vec<i32> = m.iter().map(|v| *v.get(1).unwrap()).collect();

    {
        let mut c1 = col1.clone();
        let mut c2 = col2.clone();

        c1.sort();
        c2.sort();
    
        let mut dist = 0;
        for (a,b) in c1.iter().zip(c2.iter()) {
            dist += (a-b).abs();
        }
        println!("Result: {}", dist);
    }

    {
        let val: i32 = col1.iter()
                           .map(|a| a * col2.iter().filter(|&x| x==a).count() as i32)
                           .sum();
        println!("Result2: {}", val);
    }
}
