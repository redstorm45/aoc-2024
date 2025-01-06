use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let mut rules: HashSet<(i32, i32)> = HashSet::new();
    {
        for line in contents.split('\n') {
            if line.is_empty() {
                break;
            }
            let arr: Vec<i32> = line.split('|')
                          .map(|s| s.parse::<i32>().unwrap())
                          .collect();
            rules.insert( (*arr.first().unwrap(), *arr.get(1).unwrap()) );
        }
    }

    let mut updates: Vec<Vec<i32>> = vec![];
    for line in contents.split('\n').skip(rules.len()+1) {
        if line.is_empty() {
            break;
        }
        updates.push( line.split(',')
                         .map(|s| s.parse::<i32>().unwrap())
                         .collect() );
    }

    //println!("Read {} rules and {} lists", rules.len(), update.len());

    let (valid, invalid): (Vec<_>,Vec<_>) = updates.iter().partition(|update| is_valid_update(&rules, &update));

    let valid_sum: i32 = valid
        .into_iter()
        .map(get_update_mid)
        .sum();

    let invalid_sum: i32 = invalid
        .into_iter()
        .map(|u| sorted_update(&rules, u))
        .map(|u| get_update_mid(&u))
        .sum();

    println!("Result: {}", valid_sum);
    println!("Result2: {}", invalid_sum);
}

fn is_valid_update(rules: &HashSet<(i32, i32)>, update: &Vec<i32>) -> bool {
    for i in 0..(update.len()-1) {
        let a = *update.get(i).unwrap();
        for j in (i+1)..update.len() {
            let b = *update.get(j).unwrap();
            if rules.contains(&(b, a)) {
                return false;
            }
        }
    }
    return true;
}

fn get_update_mid(update: &Vec<i32>) -> i32 {
    if update.len() % 2 == 0 {
        panic!("Even-length update");
    }
    return *update.get((update.len()-1)/2).unwrap();
}

fn sorted_update(rules: &HashSet<(i32, i32)>, update: &Vec<i32>) -> Vec<i32> {
    let mut incoming: HashMap<i32,Vec<i32>> = HashMap::new();
    for (a,b) in rules {
        if update.contains(a) && update.contains(b) {
            incoming.entry(*b).or_insert(vec![]).push(*a);
        }
    }
    let mut no_incoming: Vec<i32> = update.iter().filter(|&v| !incoming.contains_key(v)).cloned().collect();
    let mut res = vec![];
    while !no_incoming.is_empty() {
        let first = no_incoming.pop().unwrap();
        res.push(first);
        let mut to_drop: Vec<i32> = vec![];
        for (key, values) in &mut incoming {
            if let Some(targetted_index) = values.iter().position(|&v| v==first) {
                values.swap_remove(targetted_index);
                if values.is_empty() {
                    to_drop.push(*key);
                }
            }
        }
        for e in to_drop {
            no_incoming.push(e);
            incoming.remove(&e);
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rules_examples() {
        let mut rules = HashSet::new();
        rules.insert((47,53));
        rules.insert((97,13));
        rules.insert((97,61));
        rules.insert((97,47));
        rules.insert((75,29));
        rules.insert((61,13));
        rules.insert((75,53));
        rules.insert((29,13));
        rules.insert((97,29));
        rules.insert((53,29));
        rules.insert((61,53));
        rules.insert((97,53));
        rules.insert((61,29));
        rules.insert((47,13));
        rules.insert((75,47));
        rules.insert((97,75));
        rules.insert((47,61));
        rules.insert((75,61));
        rules.insert((47,29));
        rules.insert((75,13));
        rules.insert((53,13));

        assert_eq!(is_valid_update(&rules, &vec![75,47,61,53,29]), true);
        assert_eq!(is_valid_update(&rules, &vec![97,61,53,29,13]), true);
        assert_eq!(is_valid_update(&rules, &vec![75,29,13]), true);
        assert_eq!(is_valid_update(&rules, &vec![75,97,47,61,53]), false);
        assert_eq!(is_valid_update(&rules, &vec![61,13,29]), false);
        assert_eq!(is_valid_update(&rules, &vec![97,13,75,29,47]), false);
    }

    #[test]
    fn sort_examples() {
        let mut rules = HashSet::new();
        rules.insert((47,53));
        rules.insert((97,13));
        rules.insert((97,61));
        rules.insert((97,47));
        rules.insert((75,29));
        rules.insert((61,13));
        rules.insert((75,53));
        rules.insert((29,13));
        rules.insert((97,29));
        rules.insert((53,29));
        rules.insert((61,53));
        rules.insert((97,53));
        rules.insert((61,29));
        rules.insert((47,13));
        rules.insert((75,47));
        rules.insert((97,75));
        rules.insert((47,61));
        rules.insert((75,61));
        rules.insert((47,29));
        rules.insert((75,13));
        rules.insert((53,13));

        assert_eq!(sorted_update(&rules, &vec![75,97,47,61,53]), vec![97,75,47,61,53]);
        assert_eq!(sorted_update(&rules, &vec![61,13,29]), vec![61,29,13]);
        assert_eq!(sorted_update(&rules, &vec![97,13,75,29,47]), vec![97,75,47,29,13]);
    }
}
