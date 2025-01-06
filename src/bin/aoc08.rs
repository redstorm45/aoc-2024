use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::env;


struct StaticMap{
    data: Vec<Vec<char>>
}

impl StaticMap {
    fn get_height(&self) -> usize { self.data.len() }
    fn get_width(&self) -> usize { self.data.first().unwrap().len() }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum ResonnanceMode {
    Dual,
    Line
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let map = parse_map(&contents);
    let resonnances_dual = get_all_resonnance_spots(&map, ResonnanceMode::Dual);
    let resonnances_line = get_all_resonnance_spots(&map, ResonnanceMode::Line);

    println!("Result: {}", resonnances_dual.len()); // 240
    println!("Result2: {}", resonnances_line.len());  // 966 too high, 919 too low
}

fn parse_map(txt: &str) -> StaticMap {
    StaticMap{
        data: txt.split('\n').filter(|s| !s.is_empty()).map(|line| line.chars().collect()).collect()
    }
}

fn antenna_sets(map: &StaticMap) -> HashMap<char, Vec<(usize,usize)>> {
    let mut res = HashMap::new();
    for (i,row) in map.data.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c != '.' {
                res.entry(*c).or_insert(vec![]).push((i,j));
            }
        }
    }
    res
}

fn get_all_resonnance_spots(map: &StaticMap, mode: ResonnanceMode) -> HashSet<(usize,usize)> {
    let mut res = HashSet::new();
    for (_, positions) in antenna_sets(map) {
        for p in get_resonnance_spots(&positions, map.get_height(), map.get_width(), mode) {
            res.insert(p);
        }
    }
    res
}

fn get_offset_point(origin: &(usize,usize), offset: &(isize,isize), repeat: isize) -> (isize,isize) {
    (origin.0 as isize + repeat*offset.0, origin.1 as isize + repeat*offset.1)
}

fn get_resonnance_spots(antennas: &Vec<(usize,usize)>, height: usize, width: usize, mode: ResonnanceMode) -> HashSet<(usize,usize)> {
    let mut res = HashSet::new();
    for (i, a) in antennas.iter().enumerate() {
        for b in antennas.iter().skip(i+1){
            let ab = (b.0 as isize-a.0 as isize, b.1 as isize-a.1 as isize);
            //println!("Explore {:?} - {:?}", a, b);
            if mode == ResonnanceMode::Line {
                res.insert(*a);
                res.insert(*b);
            }
            {
                let mut k = 1;
                let mut tgt = get_offset_point(a, &(-ab.0, -ab.1), k);
                while tgt.0 >= 0 && tgt.1 >= 0 && tgt.0 < height as isize && tgt.1 < width as isize {
                    res.insert((tgt.0 as usize, tgt.1 as usize));
                    if mode == ResonnanceMode::Dual {
                        break;
                    }
                    k += 1;
                    tgt = get_offset_point(a, &(-ab.0, -ab.1), k);
                }
                //println!("Invalid at {:?}", &tgt);
            }
            {
                let mut k = 1;
                let mut tgt = get_offset_point(b, &ab, k);
                while tgt.0 >= 0 && tgt.1 >= 0 && tgt.0 < height as isize && tgt.1 < width as isize {
                    res.insert((tgt.0 as usize, tgt.1 as usize));
                    if mode == ResonnanceMode::Dual {
                        break;
                    }
                    k += 1;
                    tgt = get_offset_point(b, &ab, k);
                }
                //println!("Invalid2 at {:?}", &tgt);
            }
        }
    }
    //println!("Found resonance spots: {:?}", &res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_dual_simple() {
        assert_eq!(get_resonnance_spots(&vec![(3,4),(4,8),(5,5)], 10, 10, ResonnanceMode::Dual).len(), 4);
    }

    #[test]
    fn example_line() {
        assert_eq!(get_resonnance_spots(&vec![(0,0),(1,3),(2,1)], 10, 10, ResonnanceMode::Line).len(), 9);
    }

    #[test]
    fn example_full() {
        let map = parse_map("............\n........0...\n.....0......\n.......0....\n....0.......\n......A.....\n............\n............\n........A...\n.........A..\n............\n............");
    }
}