#![allow(clippy::comparison_chain)]

use std::fs;
use std::env;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::collections::hash_map::Entry;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let laby = parse_map(&contents);
    let best_infos = explore_base_paths(&laby);

    let best_path = best_path_from_infos(&laby, &best_infos);
    let cost = path_cost(&best_path);

    let benches = explored_from_infos(&laby, &best_infos).len();

    println!("Result: {}", cost);
    println!("Result2: {}", benches);
}

const COST_FORWARD: usize = 1;
const COST_TURN: usize = 1000;

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

#[derive(Clone, Copy, PartialEq, Eq, std::hash::Hash)]
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

#[derive(Clone, Copy)]
enum Action {
    TurnLeft,
    TurnRight,
    Forward,
}

struct ExploreInfo {
    parents: Vec<((usize,usize),Direction)>,
    parent_action: Vec<Action>,
    distance: usize
}

fn insert_sorted_by<T,F>(vec: &mut VecDeque<T>, elem: T, mut sort_func: F)
    where F: FnMut(&T)->usize
{
    let target_key = sort_func(&elem);
    let search_res = vec.binary_search_by_key(&target_key, &mut sort_func);

    if let Ok(i) = search_res {
        vec.insert(i, elem);
        //let distances: Vec<usize> = vec.iter().map(sort_func).collect();
        //println!("After insert ok of {} at {}: {:?}", target_key, i, distances);
    }
    else if let Err(i) = search_res {
        vec.insert(i, elem);
        //let distances: Vec<usize> = vec.iter().map(sort_func).collect();
        //println!("After insert err of {} at {}: {:?}", target_key, i, distances);
    }
}

fn explore_base_paths(map: &Map) -> HashMap<((usize,usize),Direction), ExploreInfo> {
    let mut infos: HashMap<((usize,usize),Direction), ExploreInfo> = HashMap::new();
    let mut to_explore: VecDeque<_> = VecDeque::from( [(map.start, Direction::Right)] );
    infos.insert((map.start, Direction::Right), ExploreInfo{
        parents: vec![],
        parent_action: vec![],
        distance:0
    });

    let mut prev_distance = 0;

    while !to_explore.is_empty() {
        let current = to_explore.pop_front().unwrap();
        let mut needs_sort = false;
        /*
        if current.0 == map.end {
            break;
        }
        */
        let current_distance = infos.get(&current).unwrap().distance;
        //println!("Explore at distance {}", current_distance);
        if prev_distance > current_distance {
            println!("Explore at distance {} after {}", current_distance, prev_distance);
            panic!();
        }
        prev_distance = current_distance;
        // forward
        let moved_forward = get_moved_position(current.0, map.get_size(), current.1);
        if *map.get_cell(moved_forward.unwrap()).unwrap() == Cell::Empty {
            let entry = infos.entry((moved_forward.unwrap(), current.1));
            let new_info = ExploreInfo{
                parents: vec![current],
                parent_action: vec![Action::Forward],
                distance: current_distance + COST_FORWARD,
            };
            match entry {
                Entry::Vacant(v) => {
                    v.insert(new_info);
                    insert_sorted_by(&mut to_explore, (moved_forward.unwrap(), current.1), |e| infos.get(e).unwrap().distance);
                },
                Entry::Occupied(mut o) => {
                    if o.get().distance > new_info.distance {
                        o.insert(new_info);
                        needs_sort = true;
                    } else if o.get().distance == new_info.distance {
                        o.get_mut().parents.push(current);
                        o.get_mut().parent_action.push(Action::Forward);
                    }
                }
            };
        }
        // left turn
        {
            let entry = infos.entry((current.0, current.1.turn_left()));
            let new_info = ExploreInfo{
                parents: vec![current],
                parent_action: vec![Action::TurnLeft],
                distance: current_distance + COST_TURN,
            };
            match entry {
                Entry::Vacant(v) => {
                    v.insert(new_info);
                    insert_sorted_by(&mut to_explore, (current.0, current.1.turn_left()), |e| infos.get(e).unwrap().distance);
                },
                Entry::Occupied(mut o) => {
                    if o.get().distance > new_info.distance {
                        o.insert(new_info);
                        needs_sort = true;
                    } else if o.get().distance == new_info.distance {
                        o.get_mut().parents.push(current);
                        o.get_mut().parent_action.push(Action::TurnLeft);
                    }
                }
            };
        }
        // right turn
        {
            let entry = infos.entry((current.0, current.1.turn_right()));
            let new_info = ExploreInfo{
                parents: vec![current],
                parent_action: vec![Action::TurnRight],
                distance: current_distance + COST_TURN,
            };
            match entry {
                Entry::Vacant(v) => {
                    v.insert(new_info);
                    insert_sorted_by(&mut to_explore, (current.0, current.1.turn_right()), |e| infos.get(e).unwrap().distance);
                },
                Entry::Occupied(mut o) => {
                    if o.get().distance > new_info.distance {
                        o.insert(new_info);
                        needs_sort = true;
                    } else if o.get().distance == new_info.distance {
                        o.get_mut().parents.push(current);
                        o.get_mut().parent_action.push(Action::TurnRight);
                    }
                }
            };
        }
        if needs_sort {
            to_explore.make_contiguous().sort_by_key(|e| infos.get(e).unwrap().distance);
        }
    }

    infos
}

fn best_path_from_infos(map: &Map, infos: &HashMap<((usize,usize),Direction), ExploreInfo>) -> Vec<Action> {
    // build backwards
    let mut end_vec: Vec<((usize,usize),Direction)> = infos.keys().filter(|e| e.0 == map.end).cloned().collect();
    end_vec.sort_by_key(|item| infos.get(item).unwrap().distance);
    let mut reached = end_vec.first().cloned();
    let mut res: Vec<Action> = vec![];
    while reached.is_some() {
        let pos = reached.unwrap();
        let info = infos.get(&pos).unwrap();
        reached = info.parents.first().cloned();
        if let Some(act) = info.parent_action.first() {
            res.push(*act);
        }
    }
    res.reverse();
    res
}

fn explored_from_infos(map: &Map, infos: &HashMap<((usize,usize),Direction), ExploreInfo>) -> HashSet<(usize,usize)> {
    let mut back_explored: HashSet<((usize,usize),Direction)> = HashSet::new();
    let mut wave : VecDeque<((usize,usize), Direction)> = VecDeque::new();

    let end_vec: Vec<((usize,usize),Direction)> = infos.keys().filter(|e| e.0 == map.end).cloned().collect();
    let best_score = end_vec.iter().map(|k| infos.get(k).unwrap().distance).min().unwrap();

    for posdir in infos.keys().filter(|e| e.0 == map.end) {
        if infos.get(posdir).unwrap().distance == best_score {
            wave.push_back(*posdir);
            back_explored.insert(*posdir);
        }
    }

    while !wave.is_empty() {
        let explored = wave.pop_front().unwrap();
        let ancestors = &infos.get(&explored).unwrap().parents;
        for ancestor in ancestors {
            if !back_explored.contains(ancestor) {
                back_explored.insert(*ancestor);
                wave.push_back(*ancestor);
            }
        }
    }

    let mut res = HashSet::new();
    for (pt,_) in back_explored {
        res.insert(pt);
    }
    res
}

fn path_cost(actions: &[Action]) -> usize {
    actions.iter().map(|act| match act {
        Action::Forward => COST_FORWARD,
        _ => COST_TURN
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let laby = parse_map("###############\n#.......#....E#\n#.#.###.#.###.#\n#.....#.#...#.#\n#.###.#####.#.#\n#.#.#.......#.#\n#.#.#####.###.#\n#...........#.#\n###.#.#####.#.#\n#...#.....#.#.#\n#.#.#.###.#.#.#\n#.....#...#.#.#\n#.###.#.#.#.#.#\n#S..#.....#...#\n###############\n");
        let best_infos = explore_base_paths(&laby);

        let best_path = best_path_from_infos(&laby, &best_infos);
        let cost = path_cost(&best_path);
        assert_eq!(cost, 7036);

        let bench = explored_from_infos(&laby, &best_infos);
        assert_eq!(bench.len(), 45);
    }
}