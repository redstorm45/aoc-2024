use std::env;
use std::fs;
use std::collections::HashMap;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let laby = parse_map(contents.as_str());

    let dist_start = explore_all_from(&laby, laby.start);
    let dist_end = explore_all_from(&laby, laby.end);

    let distances = get_simple_skip_distances(&laby, &dist_start, &dist_end);
    let scores = get_skip_scores(&laby, &dist_start, &distances);

    let count = scores.iter().filter(|s| s.improvement>=100).count();

    // lower than 1471
    println!("Result: {}", count);
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, std::hash::Hash)]
enum Cell {
    Empty,
    Wall,
}

#[derive(PartialEq, Eq, Debug)]
struct Map {
    cells: Vec<Vec<Cell>>,
    start: (usize,usize),
    end: (usize,usize)
}

impl Map {
    fn get_size(&self) -> (usize,usize) {
        (self.cells.len(), self.cells.first().unwrap().len())
    }
    fn get_cell(&self, pos: (usize, usize)) -> Option<&Cell> {
        self.cells.get(pos.0).and_then(|row| row.get(pos.1))
    }
}

fn parse_map(s: &str) -> Map {
    Map {
        cells: s.split_terminator('\n')
            .map(|line| line.chars().map(|c| match c {
                '#' => Cell::Wall,
                _ => Cell::Empty
            }).collect())
            .collect(),
        start: s
            .split_terminator('\n')
            .enumerate()
            .find_map(|(i, s)| {
                s.chars()
                .enumerate()
                .find_map(|(j, c)| match c {
                    'S' => Some((i,j)),
                    _ => None
                })
            }).unwrap(),
        end: s
            .split_terminator('\n')
            .enumerate()
            .find_map(|(i, s)| {
                s.chars()
                .enumerate()
                .find_map(|(j, c)| match c {
                    'E' => Some((i,j)),
                    _ => None
                })
            }).unwrap()
    }
}


#[derive(Clone, Copy, PartialEq, Eq, std::hash::Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }
}

fn get_moved_position(pos: (usize, usize), size: (usize, usize), dir: Direction) -> Option<(usize,usize)> {
    match dir {
        Direction::Down => {
            if pos.0 +1 < size.0 {
                Some((pos.0+1, pos.1))
            } else {
                None
            }
        },
        Direction::Right => {
            if pos.1 +1 < size.1 {
                Some((pos.0, pos.1+1))
            } else {
                None
            }
        },
        Direction::Left => {
            if pos.1 > 0 {
                Some((pos.0, pos.1-1))
            } else {
                None
            }
        },
        Direction::Up => {
            if pos.0 > 0 {
                Some((pos.0-1, pos.1))
            } else {
                None
            }
        },
    }
}

fn explore_all_from(map: &Map, origin: (usize,usize)) -> HashMap<(usize,usize),usize> {
    let mut res = HashMap::new();
    res.insert(origin, 0);
    let mut front = vec![origin];
    let mut distance: usize = 0;
    while !front.is_empty() {
        distance += 1;
        let mut new_front = vec![];
        for pos in front {
            for dir in [Direction::Up, Direction::Down, Direction::Right, Direction::Left] {
                if let Some(next_pos) = get_moved_position(pos, map.get_size(), dir) {
                    if *map.get_cell(next_pos).unwrap() == Cell::Wall {
                        continue
                    }
                    if let std::collections::hash_map::Entry::Vacant(v) = res.entry(next_pos) {
                        v.insert(distance);
                        new_front.push(next_pos);
                    }
                }
            }
        }
        front = new_front;
    }
    res
}

struct SkipResult {
    source: (usize,usize),
    destination: (usize,usize),
    total_dist: usize
}

fn get_simple_skip_distances(map: &Map, dist_start: &HashMap<(usize,usize),usize>, dist_end: &HashMap<(usize,usize),usize>) -> Vec<SkipResult> {
    let mut res = vec![];

    for i in 0..map.get_size().0 {
        for j in 0..map.get_size().1 {
            if *map.get_cell((i,j)).unwrap() == Cell::Wall {
                let to_left = get_moved_position((i,j), map.get_size(), Direction::Left);
                let to_right = get_moved_position((i,j), map.get_size(), Direction::Right);

                if let (Some(pos_left),Some(pos_right)) = (to_left,to_right) {
                    if let (Some(dsl),Some(der)) = (dist_start.get(&pos_left), dist_end.get(&pos_right)) {
                        res.push(SkipResult{
                            source: pos_left,
                            destination: pos_right,
                            total_dist: dsl + der +2
                        })
                    }
                    if let (Some(dsr),Some(del)) = (dist_start.get(&pos_right), dist_end.get(&pos_left)) {
                        res.push(SkipResult{
                            source: pos_right,
                            destination: pos_left,
                            total_dist: dsr + del +2
                        })
                    }
                }

                let to_up = get_moved_position((i,j), map.get_size(), Direction::Up);
                let to_down = get_moved_position((i,j), map.get_size(), Direction::Down);

                if let (Some(pos_up),Some(pos_down)) = (to_up,to_down) {
                    if let (Some(dsu),Some(ded)) = (dist_start.get(&pos_up), dist_end.get(&pos_down)) {
                        res.push(SkipResult{
                            source: pos_up,
                            destination: pos_down,
                            total_dist: dsu + ded +2
                        })
                    }
                    if let (Some(dsd),Some(deu)) = (dist_start.get(&pos_down), dist_end.get(&pos_up)) {
                        res.push(SkipResult{
                            source: pos_down,
                            destination: pos_up,
                            total_dist: dsd + deu +2
                        })
                    }
                }
            }
        }
    }

    res
}

fn get_full_skip_distances(map: &Map, dist_start: &HashMap<(usize,usize),usize>, dist_end: &HashMap<(usize,usize),usize>, max_jump: usize) -> Vec<SkipResult> {
    let mut res = vec![];

    let dist_jumps: Vec<(isize, isize)> = (-(max_jump as isize)..max_jump as isize).flat_map(|i| {
        (-(max_jump as isize)..max_jump as isize).map(move |j| (i,j))
    }).filter(|(i,j)| ((i.abs()+j.abs()) as usize <= max_jump) && (*i,*j) != (0,0)).collect();

    for (src_pos, src_dist) in dist_start {
        let i = src_pos.0 as isize;
        let j = src_pos.1 as isize;
        for (di,dj) in dist_jumps.iter() {
            if i+di >= 0 && i+di < map.get_size().0 as isize && j+dj >= 0 && j+dj < map.get_size().1 as isize {
                let new_pos = ((i+di) as usize, (j+dj) as usize);
                if let Some(dst_dist) = dist_end.get(&new_pos) {
                    res.push(SkipResult{
                        source: *src_pos,
                        destination: new_pos,
                        total_dist: src_dist + dst_dist
                    });
                }
            }
        }
    }

    res
}

#[derive(Debug)]
struct SkipScore {
    source: (usize,usize),
    destination: (usize,usize),
    improvement: usize
}

fn get_skip_scores(map: &Map, dist_start: &HashMap<(usize,usize),usize>, dist: &[SkipResult]) -> Vec<SkipScore> {
    let best_dist = *dist_start.get(&map.end).unwrap();
    dist.iter().filter(|sr| sr.total_dist < best_dist).map(|sr| SkipScore{
        source: sr.source,
        destination: sr.destination,
        improvement: best_dist-sr.total_dist
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let laby = parse_map("###############\n#...#...#.....#\n#.#.#.#.#.###.#\n#S#...#.#.#...#\n#######.#.#.###\n#######.#.#...#\n#######.#.###.#\n###..E#...#...#\n###.#######.###\n#...###...#...#\n#.#####.#.###.#\n#.#...#.#.#...#\n#.#.#.#.#.#.###\n#...#...#...###\n###############");

        let dist_start = explore_all_from(&laby, laby.start);
        let dist_end = explore_all_from(&laby, laby.end);

        let distances = get_simple_skip_distances(&laby, &dist_start, &dist_end);
        let mut scores: Vec<usize> = get_skip_scores(&laby, &dist_start, &distances).iter().map(|ss| ss.improvement).collect();

        scores.sort();

        assert_eq!(*dist_start.get(&laby.end).unwrap(),84);

        assert_eq!(scores, vec![
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
            6, 6,
            8, 8, 8, 8,
            10, 10,
            12, 12, 12,
            20, 36, 38, 40, 64
        ]);

        let distances2 = get_full_skip_distances(&laby, &dist_start, &dist_end, 20);
        let mut scores2: Vec<usize> = get_skip_scores(&laby, &dist_start, &distances2).iter().map(|ss| ss.improvement).collect();

        scores2.sort();

        println!("{:?}", scores2);

        assert_eq!(scores2.iter().filter(|e| **e == 50).count(), 32);
        assert_eq!(scores2.iter().filter(|e| **e == 52).count(), 29);
        assert_eq!(scores2.iter().filter(|e| **e == 54).count(), 29);
        assert_eq!(scores2.iter().filter(|e| **e == 56).count(), 39);
        assert_eq!(scores2.iter().filter(|e| **e == 58).count(), 25);
        assert_eq!(scores2.iter().filter(|e| **e == 60).count(), 23);
        assert_eq!(scores2.iter().filter(|e| **e == 62).count(), 20);
        assert_eq!(scores2.iter().filter(|e| **e == 64).count(), 19);
        assert_eq!(scores2.iter().filter(|e| **e == 66).count(), 12);
        assert_eq!(scores2.iter().filter(|e| **e == 68).count(), 14);
        assert_eq!(scores2.iter().filter(|e| **e == 70).count(), 12);
        assert_eq!(scores2.iter().filter(|e| **e == 72).count(), 22);
        assert_eq!(scores2.iter().filter(|e| **e == 74).count(), 4);
        assert_eq!(scores2.iter().filter(|e| **e == 76).count(), 3);
    }
}