use std::fs;
use std::env;
use std::collections::HashSet;
use std::collections::HashMap;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let map = parse_map(&contents);
    let regions = split_regions(&map);

    let score = total_cost(&regions);
    let score2 = total_discount_cost(&regions);

    println!("Result: {}", score);
    println!("Result2: {}", score2);
}

fn parse_map(s: &str) -> Vec<Vec<char>> {
    s.split_terminator('\n').map(|k| {
        k.chars().collect()
    }).collect()
}

fn merge_groups<T>(groups: &mut HashMap<usize,HashSet<T>>, revgroups: &mut HashMap<T,usize>, a: usize, b:usize) -> usize
where T: Eq, T: std::hash::Hash, T: Clone
{
    if a == b {
        return a;
    }
    //println!("Merge groups {} and {}", a, b);
    let target_group = a.min(b);
    let source_group = a.max(b);

    let source = groups.remove(&source_group).unwrap();
    let target = groups.get_mut(&target_group).unwrap();
    for e in source.into_iter() {
        target.insert(e.clone());
        revgroups.insert(e, target_group);
    }

    target_group
}

fn split_regions(map: &Vec<Vec<char>>) -> Vec<HashSet<(usize,usize)>>{
    let mut group_by_coord: HashMap<(usize,usize),usize> = HashMap::new();
    let mut coord_by_group: HashMap<usize,HashSet<(usize,usize)>> = HashMap::new();

    // give a group to each pixel
    for (i,line) in map.iter().enumerate() {
        for (j, _) in line.iter().enumerate() {
            coord_by_group.insert(group_by_coord.len(), HashSet::from([(i,j)]));
            group_by_coord.insert((i,j), group_by_coord.len());
        }
    }

    // connect right & down
    for (i,line) in map.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if i+1<map.len() && map[i+1][j] == *c {
                let groupa: usize = *group_by_coord.get(&(i,j)).unwrap();
                let groupb: usize = *group_by_coord.get(&(i+1,j)).unwrap();
                let newgroup = merge_groups(&mut coord_by_group, &mut group_by_coord, groupa, groupb);
            }
            if j+1<line.len() && map[i][j+1] == *c {
                let groupa: usize = *group_by_coord.get(&(i,j)).unwrap();
                let groupb: usize = *group_by_coord.get(&(i,j+1)).unwrap();
                let newgroup = merge_groups(&mut coord_by_group, &mut group_by_coord, groupa, groupb);
            }
        }
    }

    // connect left & up
    for (i,line) in map.iter().enumerate().rev() {
        for (j, c) in line.iter().enumerate().rev() {
            if i>0 && map[i-1][j] == *c {
                let groupa: usize = *group_by_coord.get(&(i,j)).unwrap();
                let groupb: usize = *group_by_coord.get(&(i-1,j)).unwrap();
                let newgroup = merge_groups(&mut coord_by_group, &mut group_by_coord, groupa, groupb);
            }
            if j>0 && map[i][j-1] == *c {
                let groupa: usize = *group_by_coord.get(&(i,j)).unwrap();
                let groupb: usize = *group_by_coord.get(&(i,j-1)).unwrap();
                let newgroup = merge_groups(&mut coord_by_group, &mut group_by_coord, groupa, groupb);
            }
        }
    }

    coord_by_group.values().cloned().collect()
}

fn perimeter(region: &HashSet<(usize,usize)>) -> usize {
    // suppose the region has no holes
    // count edges to each side
    let mut res = 0;
    for (di,dj) in [(1,0), (-1_isize,0), (0,1), (0,-1_isize)] {
        for &(i,j) in region.iter() {
            let (ni,nj) = ((i as isize)+di, (j as isize)+dj);
            if ni < 0 || nj < 0 || !region.contains(&(ni as usize, nj as usize)) {
                res += 1;
            }
        }
    }
    res
}

#[derive(PartialEq,Eq,std::hash::Hash,Clone,Copy,Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn sides(region: &HashSet<(usize,usize)>) -> usize {
    let mut segments = HashSet::new();
    for (di,dj, dd) in [(1,0,Direction::Right), (-1_isize,0,Direction::Left), (0,1,Direction::Down), (0,-1_isize,Direction::Up)] {
        for &(i,j) in region.iter() {
            let (ni,nj) = ((i as isize)+di, (j as isize)+dj);
            if ni < 0 || nj < 0 || !region.contains(&(ni as usize, nj as usize)) {
                segments.insert((i as isize,j as isize,dd));
            }
        }
    }
    //println!("Segments of {:?} : {:?}", region, segments);
    
    let mut res = 0;
    while !segments.is_empty() {
        let one_seg = *segments.iter().next().unwrap();
        segments.remove(&one_seg);
        let mut seg_length= 1;
        if one_seg.2 == Direction::Up || one_seg.2 == Direction::Down {
            let mut farthest_left = one_seg;
            let mut farthest_right = one_seg;
            // move left
            while segments.contains(&(farthest_left.0-1, farthest_left.1, one_seg.2)) {
                farthest_left = (farthest_left.0-1, farthest_left.1, one_seg.2);
                segments.remove(&farthest_left);
            }
            // then right
            while segments.contains(&(farthest_right.0+1, farthest_right.1, one_seg.2)) {
                farthest_right = (farthest_right.0+1, farthest_right.1, one_seg.2);
                segments.remove(&farthest_right);
            }
        }
        else if one_seg.2 == Direction::Left || one_seg.2 == Direction::Right {
            let mut farthest_up = one_seg;
            let mut farthest_down = one_seg;
            // move left
            while segments.contains(&(farthest_up.0, farthest_up.1-1, one_seg.2)) {
                farthest_up = (farthest_up.0, farthest_up.1-1, one_seg.2);
                segments.remove(&farthest_up);
            }
            // then right
            while segments.contains(&(farthest_down.0, farthest_down.1+1, one_seg.2)) {
                farthest_down = (farthest_down.0, farthest_down.1+1, one_seg.2);
                segments.remove(&farthest_down);
            }
        }
        res += 1;
    }
    res
}

fn total_cost(regions: &[HashSet<(usize,usize)>]) -> usize {
    regions.iter().map(|r| r.len()*perimeter(r)).sum()
}

fn total_discount_cost(regions: &[HashSet<(usize,usize)>]) -> usize {
    regions.iter().map(|r| r.len()*sides(r)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_split() {
        let map = parse_map("OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO");
        let regions = split_regions(&map);
        assert_eq!(regions.len(), 5);
    }

    #[test]
    fn example_simple() {
        let map = parse_map("AAAA\nBBCD\nBBCC\nEEEC");
        let regions = split_regions(&map);
        for r in regions.iter() {
            println!("{:?} {}", r, perimeter(r));
        }
        //println!("{:?}", regions);
        assert_eq!(total_cost(&regions),140);
        assert_eq!(total_discount_cost(&regions),80);
    }
}
