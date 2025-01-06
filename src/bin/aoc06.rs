use std::collections::HashSet;
use std::fs;
use std::env;
use std::fmt::{Display,Formatter,Result};


#[derive(PartialEq, Eq)]
enum Tile{
    Empty,
    Wall
}

#[derive(PartialEq, Eq)]
enum MoveResult {
    Void,
    Loop
}

trait Map {
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn is_wall(&self, x:usize, y:usize) -> bool;
}

struct StaticMap {
    data: Vec<Vec<Tile>>
}

impl Map for StaticMap {
    fn get_width(&self) -> usize {
        self.data.len()
    }
    fn get_height(&self) -> usize {
        self.data.first().unwrap().len()
    }
    fn is_wall(&self, x:usize, y:usize) -> bool {
        *self.data.get(x).unwrap().get(y).unwrap() == Tile::Wall
    }
}

struct OverlayMap<'a>{
    data: &'a StaticMap,
    added: (usize,usize)
}

impl Map for OverlayMap<'_> {
    fn get_width(&self) -> usize {
        self.data.get_width()
    }
    fn get_height(&self) -> usize {
        self.data.get_height()
    }
    fn is_wall(&self, x:usize, y:usize) -> bool {
        (x, y) == self.added || self.data.is_wall(x, y)
    }
}


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction{
    Up,
    Down,
    Left,
    Right
}

fn next_direction(d: Direction) -> Direction {
    match d {
        Direction::Up => Direction::Right,
        Direction::Left => Direction::Up,
        Direction::Down => Direction::Left,
        Direction::Right => Direction::Down,
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let map = parse_map(&contents);

    let coords = contents
        .split('\n')
        .filter(|s| !s.is_empty())
        .enumerate()
        .find_map(|(i, s)| {
            s.chars()
            .enumerate()
            .find_map(|(j, c)| match c {
                '>' => Some((j, Direction::Right)),
                '^' => Some((j, Direction::Up)),
                'v' => Some((j, Direction::Down)),
                '<' => Some((j, Direction::Left)),
                _ => None
            })
            .map(|(j,d)| (i,j,d))
        });

    let visited = get_visited(&map, coords.unwrap()).len();
    let blockers = get_loop_options(&map, coords.unwrap()).len();

    println!("Result: {}", visited);
    println!("Result2: {}", blockers);
}

fn parse_map(data: &str) -> StaticMap {
    StaticMap{
        data: data
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.chars()
                  .map(|c| match c {
                    '#' => Tile::Wall,
                    _ => Tile::Empty
                  })
                  .collect())
        .collect()
    }
}

fn get_visited(map: &dyn Map, (x, y, direction): (usize, usize, Direction)) -> HashSet<(usize,usize)> {
    let mut coords = Some((x,y,direction));
    let mut visited: HashSet<(usize,usize)> = HashSet::new();

    while coords.is_some() {
        let (x, y, d) = coords.unwrap();
        visited.insert((x,y));
        coords = move_one_step(map, (x, y, d));
    }

    visited
}

fn move_until_stopped(map: &dyn Map, start: (usize, usize, Direction), explored: &HashSet<(usize,usize,Direction)>) -> MoveResult {
    let mut new_explored:HashSet<(usize,usize,Direction)> = HashSet::new();

    let mut coords = Some(start);
    while coords.is_some() {
        let (x, y, d) = coords.unwrap();
        if explored.contains(&(x,y,d)) || new_explored.contains(&(x,y,d)) {
            return MoveResult::Loop;
        } else {
            new_explored.insert((x,y,d));
        }
        coords = move_one_step_far(map, (x, y, d));
    }
    MoveResult::Void
}

fn get_loop_options(map: &StaticMap, (x, y, direction): (usize, usize, Direction)) -> HashSet<(usize,usize)> {
    let width = map.get_width();
    let height = map.get_height();

    let mut coords = Some((x,y,direction));
    let mut entry_pos: HashSet<(usize,usize,Direction)> = HashSet::new();

    let mut blocker_options = HashSet::new();

    while coords.is_some() {
        let (x, y, d) = coords.unwrap();
        entry_pos.insert((x,y,d));

        //println!("Explore {} {} {}", x, y, d);

        if let Some((fx,fy)) = get_moved(x, y, d, width, height) {
            if !map.is_wall(fx, fy)
                    && !entry_pos.contains(&(fx,fy,next_direction(d)))
                    && !entry_pos.contains(&(fx,fy,next_direction(next_direction(next_direction(d)))))
                    && !blocker_options.contains(&(fx,fy)) {
                // try to put a wall in front
                let with_wall = OverlayMap{
                    data: map,
                    added: (fx, fy)
                };
                if move_until_stopped(&with_wall, (x, y, next_direction(d)), &entry_pos) == MoveResult::Loop {
                    blocker_options.insert((fx,fy));
                }
            }
        }
        coords = move_one_step(map, (x, y, d));
    }

    blocker_options
}

fn get_moved(x: usize, y: usize, direction: Direction, width: usize, height: usize) -> Option<(usize,usize)>{
    match direction {
        Direction::Down => {
            if x+1 < width {
                Some((x+1, y))
            } else {
                None
            }
        },
        Direction::Up => {
            if x > 0 {
                Some((x-1, y))
            } else {
                None
            }
        },
        Direction::Right => {
            if y+1 < height {
                Some((x, y+1))
            } else {
                None
            }
        },
        Direction::Left => {
            if y > 0 {
                Some((x, y-1))
            } else {
                None
            }
        }
    }
}

fn move_one_step(map: &dyn Map, (x, y, direction): (usize, usize, Direction)) -> Option<(usize,usize,Direction)> {
    let width = map.get_width();
    let height = map.get_height();
    let next_pos = get_moved(x, y, direction, width, height);

    next_pos.map(|(nx,ny)| {
        match map.is_wall(nx, ny) {
            false => (nx,ny,direction),
            true => (x,y,next_direction(direction))
        }
    })
}

fn move_one_step_far(map: &dyn Map, (x, y, direction): (usize, usize, Direction)) -> Option<(usize,usize,Direction)> {
    // move until a wall collision causing a turn, or a void-out
    let width = map.get_width();
    let height = map.get_height();

    let mut prev = (x,y);
    let mut next_pos = get_moved(x, y, direction, width, height);

    while next_pos.is_some_and(|(nx,ny)| !map.is_wall(nx, ny)) {
        prev = (next_pos.unwrap().0, next_pos.unwrap().1);
        next_pos = get_moved(prev.0, prev.1, direction, width, height);
    }

    next_pos.map(|(_,_)| (prev.0, prev.1, next_direction(direction)))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_line() {
        let map = parse_map("......\n......");
        assert_eq!(get_visited(&map, (1,0,Direction::Right)).len(), 6);
    }

    #[test]
    fn simple_turn() {
        let map = parse_map("....#.\n......");
        assert_eq!(get_visited(&map, (0,0,Direction::Right)).len(), 5);
    }

    #[test]
    fn loop_outside_square() {
        let map = parse_map("....#.\n.#....\n...#..");
        assert_eq!(get_loop_options(&map, (0,0,Direction::Right)).len(), 0);
    }

    #[test]
    fn loop_small_square() {
        let map = parse_map("......\n....#.\n.#....\n...#..");
        assert_eq!(get_loop_options(&map, (1,0,Direction::Right)).len(), 1);
    }

    #[test]
    fn loop_second_order() {
        // Loop needs secondary collision to be taken into account
        /*
          .#.....
          >...#..
          #......
          ...O...
         */
        let map = parse_map(".#.....\n....#..\n#......\n.......");
        assert_eq!(get_loop_options(&map, (1,0,Direction::Right)).len(), 1);
    }

    #[test]
    fn example() {
        let map = parse_map("....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#........\n........#.\n#.........\n......#...");
        assert_eq!(get_visited(&map, (6,4,Direction::Up)).len(), 41);
        assert_eq!(get_loop_options(&map, (6,4,Direction::Up)).len(), 6);
    }
}

