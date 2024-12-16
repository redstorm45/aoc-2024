use std::fs;
use std::env;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::hash_map::Entry;

fn main() {
    println!("Hello, world!");
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
    fn get_cell_mut(&mut self, pos: (usize, usize)) -> Option<&mut Cell> {
        self.cells.get_mut(pos.0).and_then(|row| row.get_mut(pos.1))
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

enum Action {
    TurnLeft,
    TurnRight,
    Forward,
}

struct ExploreInfo {
    parent: Option<((usize,usize),Direction)>,
    distance: usize
}

fn insert_sorted_by<T,F>(vec: &mut VecDeque<T>, elem: T, mut sort_func: F)
    where F: FnMut(&T)->usize
{
    let target_key = sort_func(&elem);
    let search_res = vec.binary_search_by_key(&target_key, sort_func);
    let target_index = search_res.unwrap();
    vec.insert(target_index, elem);
}

fn best_path(map: &Map) -> Vec<Action> {
    let mut infos: HashMap<((usize,usize),Direction), ExploreInfo> = HashMap::new();
    let mut to_explore: VecDeque<_> = VecDeque::from( [(map.start, Direction::Right)] );
    infos.insert((map.start, Direction::Right), ExploreInfo{parent: None, distance:0});

    const COST_FORWARD: usize = 1;
    const COST_TURN: usize = 1000;

    while !to_explore.is_empty() {
        let current = to_explore.pop_front().unwrap();
        let current_distance = infos.get(&current).unwrap().distance;
        // forward
        let moved_forward = get_moved_position(current.0, map.get_size(), current.1);
        if *map.get_cell(moved_forward.unwrap()).unwrap() == Cell::Empty {
            let entry = infos.entry((moved_forward.unwrap(), current.1));
            let new_info = ExploreInfo{
                parent: Some(current),
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
                    }
                }
            };
        }
        // left
        {
            let entry = infos.entry((current.0, current.1.turn_left()));
            let new_info = ExploreInfo{
                parent: Some(current),
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
                    }
                }
            };
        }
    }


    // build backwards
    let mut res: Vec<Action> = vec![];
    res
}