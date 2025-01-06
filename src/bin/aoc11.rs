use std::fs;
use std::env;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let stones: Vec<usize> = contents.strip_suffix('\n').unwrap().split(' ').map(|s| s.parse().unwrap()).collect();
    let value = count_after_progression(&stones, 25);
    let value2 = count_after_progression(&stones, 75);

    println!("Result: {}", value);
    println!("Result2: {}", value2);
}

fn split_equal(n: usize) -> Option<(usize,usize)> {
    let s = n.to_string();
    if s.len()%2 == 0 {
        let (a,b) = s.split_at(s.len()/2);
        return Some((a.parse().unwrap(), b.parse().unwrap()));
    } else {
        return None;
    }
}

fn count_instances(stone: usize, steps: usize, cache: &mut HashMap<(usize,usize),usize>) -> usize {
    if let Some(&value) = cache.get(&(stone,steps)) {
        return value;
    }
    let value;
    if steps == 0 {
        value = 1
    } else if stone == 0 {
        value = count_instances(1, steps-1, cache)
    } else if let Some((left,right)) = split_equal(stone) {
        value = count_instances(left, steps-1, cache) + count_instances(right, steps-1, cache)
    } else {
        value = count_instances(stone*2024, steps-1, cache)
    }
    cache.insert((stone,steps), value);
    value
}

fn count_after_progression(stones: &[usize], steps: usize) -> usize {
    let mut cache = HashMap::new();
    stones.iter().map(|s| count_instances(*s, steps, &mut cache)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(count_after_progression(&[125,17], 6), 22);
        assert_eq!(count_after_progression(&[125,17], 25), 55312);
    }
}