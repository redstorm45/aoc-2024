use std::fs;
use std::env;
use std::collections::{VecDeque,HashMap};
use std::collections::hash_map::Entry;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let mut laby = Map::empty(71, 71);
    let allBlocks = parse_pos(&contents);
    for (i,j) in allBlocks[..1024].iter() {
        *laby.get_cell_mut((*j,*i)).unwrap() = Cell::Wall;
    }

    let shortest_info = explore_base_paths(&laby);
    let path = best_path_from_infos(&laby, &shortest_info);

    let shortestPathLen = path.iter().filter(|a| **a == Action::Forward).count();

    let first_blocker = first_blocker(laby, &allBlocks);

    println!("Result: {}", shortestPathLen);
    println!("Result2: {:?}", first_blocker);
}

fn parse_pos(s: &str) -> Vec<(usize,usize)> {
    s.split_terminator("\n")
        .map(|line| {
            let mut it = line.split(",");
            (it.next().unwrap().parse().unwrap(), it.next().unwrap().parse().unwrap())
        })
        .collect()
}

const COST_FORWARD: usize = 1;
const COST_TURN: usize = 0;

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
    fn empty(height: usize, width: usize) -> Map {
        Map{
            cells: (0..height)
                .map(|_| (0..width).map(|_| Cell::Empty).collect())
                .collect(),
            start: (0,0),
            end: (height-1,width-1)
        }
    }
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

#[derive(Clone, Copy, PartialEq, Eq)]
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
        if let Some(pos_forward) = moved_forward {
            if *map.get_cell(moved_forward.unwrap()).unwrap() == Cell::Empty {
                let entry = infos.entry((pos_forward, current.1));
                let new_info = ExploreInfo{
                    parents: vec![current],
                    parent_action: vec![Action::Forward],
                    distance: current_distance + COST_FORWARD,
                };
                match entry {
                    Entry::Vacant(v) => {
                        v.insert(new_info);
                        insert_sorted_by(&mut to_explore, (pos_forward, current.1), |e| infos.get(e).unwrap().distance);
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
                    } else if o.get().distance == new_info.distance && COST_TURN != 0 {
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
                    } else if o.get().distance == new_info.distance && COST_TURN != 0 {
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

fn best_path_pos_from_infos(map: &Map, infos: &HashMap<((usize,usize),Direction), ExploreInfo>) -> Vec<(usize,usize)> {
    // build backwards
    let mut end_vec: Vec<((usize,usize),Direction)> = infos.keys().filter(|e| e.0 == map.end).cloned().collect();
    end_vec.sort_by_key(|item| infos.get(item).unwrap().distance);
    let mut reached = end_vec.first().cloned();
    let mut res: Vec<(usize,usize)> = vec![map.end];
    while reached.is_some() {
        let pos = reached.unwrap();
        let info = infos.get(&pos).unwrap();
        reached = info.parents.first().cloned();
        if let Some(act) = info.parent_action.first() {
            if *act == Action::Forward {
                res.push(reached.unwrap().0);
            }
        }
    }
    res.reverse();
    res
}

fn first_blocker(starting_map: Map, blocks: &[(usize,usize)]) -> (usize,usize) {
    let mut map = starting_map;
    let shortest_info = explore_base_paths(&map);
    let mut shortest_path_pos = best_path_pos_from_infos(&map, &shortest_info);
    
    let mut current_block = 0;
    while current_block+1 < blocks.len() {
        current_block += 1;
        let block_pos = blocks[current_block];
        *map.get_cell_mut(block_pos).unwrap() = Cell::Wall;
        if shortest_path_pos.contains(&block_pos) {
            // block lies on best path: recompute
            let shortest_info = explore_base_paths(&map);
            if shortest_info.keys().filter(|(p,_)| *p==map.end).count() > 0 {
                shortest_path_pos = best_path_pos_from_infos(&map, &shortest_info);
            } else {
                return block_pos;
            }
        }
    }
    return (555,555);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut laby = Map::empty(7, 7);
        for (i,j) in parse_pos("5,4\n4,2\n4,5\n3,0\n2,1\n6,3\n2,4\n1,5\n0,6\n3,3\n2,6\n5,1\n1,2\n5,5\n2,5\n6,5\n1,4\n0,4\n6,4\n1,1\n6,1\n1,0\n0,5\n1,6\n2,0")[..12].iter() {
            *laby.get_cell_mut((*j,*i)).unwrap() = Cell::Wall;
        }

        println!("{:?}", laby);

        let shortest_info = explore_base_paths(&laby);
        let path = best_path_from_infos(&laby, &shortest_info);

        assert_eq!(path.iter().filter(|a| **a == Action::Forward).count(), 22);
    }

    #[test]
    fn example_block() {
        let laby = Map::empty(7, 7);
        let blocks = parse_pos("5,4\n4,2\n4,5\n3,0\n2,1\n6,3\n2,4\n1,5\n0,6\n3,3\n2,6\n5,1\n1,2\n5,5\n2,5\n6,5\n1,4\n0,4\n6,4\n1,1\n6,1\n1,0\n0,5\n1,6\n2,0");

        let first = first_blocker(laby, &blocks);

        assert_eq!(first, (6,1));
    }
}
