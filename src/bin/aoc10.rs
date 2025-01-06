use std::collections::HashMap;
use std::fs;
use std::env;
use std::ops::AddAssign;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let map = parse_map(&contents);
    let source_info = full_propagate_path_down(&map);
    let score_simple = score_propagation_number(&source_info);
    let score_complex = score_propagation_ways(&source_info);

    println!("Result: {}", score_simple);
    println!("Result2: {}", score_complex);
}

fn parse_map(s: &str) -> Vec<Vec<i32>> {
    s.split('\n').filter(|k| !k.is_empty()).map(|k| {
        k.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()
    }).collect()
}

fn filter_map(map: &Vec<Vec<i32>>, target: i32) -> Vec<(usize,usize)> {
    map.iter().enumerate().flat_map(|(i,line)| 
        line.iter()
            .enumerate()
            .filter(|(_,v)| **v == target)
            .map(move |(j,_)| (i,j))
    ).collect()
}

fn get_adjacents(point: &(usize,usize)) -> Vec<(usize,usize)> {
    let mut res = vec![
        (point.0+1,point.1),
        (point.0,point.1+1)
    ];
    if point.0 > 0 {
        res.push((point.0-1, point.1));
    }
    if point.1 > 0 {
        res.push((point.0, point.1-1));
    }
    res
}

fn propagate_path_down(one_up_info: HashMap<(usize,usize),HashMap<usize,usize>>, layer_down: &Vec<(usize,usize)>) -> HashMap<(usize,usize),HashMap<usize,usize>> {
    // we know what ends can be reached from one level up
    // propagate that info to the next level down
    let mut res = HashMap::new();
    for tgt in layer_down {
        let mut collected = HashMap::new();
        for adj in get_adjacents(tgt) {
            if let Some(up) = one_up_info.get(&adj) {
                for (tgt, tgt_paths) in up {
                    collected.entry(*tgt).or_insert(0).add_assign(tgt_paths);
                }
            }
        }
        res.insert(*tgt, collected);
    }
    res
}

fn full_propagate_path_down(map: &Vec<Vec<i32>>) -> HashMap<(usize,usize),HashMap<usize,usize>> {
    let mut reachable: HashMap<(usize,usize),HashMap<usize,usize>>  = 
        filter_map(map, 9).iter()
            .enumerate()
            .map(|(i,&pos)| (pos, HashMap::from([(i,1)])))
            .collect();

    for height in (0..9).into_iter().rev() {
        reachable = propagate_path_down(reachable, &filter_map(map, height));
    }

    reachable
}

fn score_propagation_number(sources_counts: &HashMap<(usize,usize),HashMap<usize,usize>>) -> usize {
    sources_counts
        .iter()
        .map(|(_,m)| m.len())
        .sum()
}

fn score_propagation_ways(sources_counts: &HashMap<(usize,usize),HashMap<usize,usize>>) -> usize {
    sources_counts
        .iter()
        .flat_map(|(_,m)| m.values())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let map = parse_map("89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732");
        let source_info = full_propagate_path_down(&map);
        let score_simple = score_propagation_number(&source_info);
        let score_complex = score_propagation_ways(&source_info);

        assert_eq!(score_simple, 36);
        assert_eq!(score_complex, 81);
    }
}