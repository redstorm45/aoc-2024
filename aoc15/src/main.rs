use std::fs;
use std::env;
use std::collections::HashSet;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let mut split_str = contents.split("\n\n");
    let mut map = parse_map(split_str.next().unwrap());
    let mut double_map = double_map(&map);
    let moves = parse_directions(split_str.next().unwrap());

    for m in moves.clone() {
        try_move(&mut map, m);
    }
    for m in moves {
        try_move(&mut double_map, m);
    }

    let gps_sum = get_checksum(&map);
    let gps_sum_big = get_checksum(&double_map);

    println!("Result: {}", gps_sum);
    println!("Result2: {}", gps_sum_big);
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, std::hash::Hash)]
enum Cell {
    Empty,
    Wall,
    Box,
    BoxLeft,
    BoxRight
}

#[derive(PartialEq, Eq, Debug)]
struct Map {
    cells: Vec<Vec<Cell>>,
    bot: (usize,usize)
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
    fn _repr(&self) -> String {
        let mut res: String = String::new();
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                if self.bot == (i,j) {
                    res += "@";
                } else if self.cells[i][j] == Cell::Wall {
                    res += "#";
                } else if self.cells[i][j] == Cell::Box {
                    res += "O";
                } else if self.cells[i][j] == Cell::BoxLeft {
                    res += "[";
                } else if self.cells[i][j] == Cell::BoxRight {
                    res += "]";
                } else {
                    res += ".";
                }
            }
            res += "\n";
        }
        res
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn parse_map(s: &str) -> Map {
    Map {
        cells: s.split_terminator('\n')
            .map(|line| line.chars().map(|c| match c {
                '#' => Cell::Wall,
                'O' => Cell::Box,
                _ => Cell::Empty
            }).collect())
            .collect(),
        bot: s
            .split_terminator('\n')
            .enumerate()
            .find_map(|(i, s)| {
                s.chars()
                .enumerate()
                .find_map(|(j, c)| match c {
                    '@' => Some((i,j)),
                    _ => None
                })
            }).unwrap()
    }
}

fn double_map(map: &Map) -> Map {
    Map {
        cells: map.cells.iter().map(|row| row.iter().flat_map(|c| {
            match c {
                Cell::Box => [Cell::BoxLeft, Cell::BoxRight].iter(),
                Cell::Wall => [Cell::Wall, Cell::Wall].iter(),
                Cell::Empty => [Cell::Empty, Cell::Empty].iter(),
                _ => [].iter(),
            }
        }).cloned().collect()).collect(),
        bot: (map.bot.0, map.bot.1*2)
    }
}


fn parse_directions(s: &str) -> Vec<Direction> {
    s.chars().filter_map(|c| match c {
        'v' => Some(Direction::Down),
        '>' => Some(Direction::Right),
        '<' => Some(Direction::Left),
        '^' => Some(Direction::Up),
        _ => None
    }).collect()
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

fn get_pushed_boxes(map: &Map, dir: Direction) -> Option<Vec<(usize,usize)>>{
    // returns None if boxes can't be pushed, else the list of cells containing pushed boxes
    let mut boxes = vec![];
    
    let mut to_explore = vec![map.bot];
    while !to_explore.is_empty() {
        //println!("Discover {:?}", to_explore);
        let mut joined_to_explore = HashSet::new();
        for e in to_explore {
            joined_to_explore.insert(e);
            if dir == Direction::Up || dir == Direction::Down {
                if *map.get_cell(e).unwrap() == Cell::BoxLeft {
                    joined_to_explore.insert(get_moved_position(e, map.get_size(), Direction::Right).unwrap());
                } else if *map.get_cell(e).unwrap() == Cell::BoxRight {
                    joined_to_explore.insert(get_moved_position(e, map.get_size(), Direction::Left).unwrap());
                }
            }
        }

        let mut new_to_explore = vec![];
        for expl in joined_to_explore {
            boxes.push(expl);
            let next = get_moved_position(expl, map.get_size(), dir).unwrap();
            if *map.get_cell(next).unwrap() == Cell::Wall {
                return None;
            } else if *map.get_cell(next).unwrap() == Cell::Empty {
            } else {
                new_to_explore.push(next);
            }
        }
        to_explore = new_to_explore;
    }

    Some(boxes)
}

fn try_move(map: &mut Map, dir: Direction) {
    if let Some(boxes) = get_pushed_boxes(map, dir) {
        let mut moved_boxes: HashSet<((usize,usize), Cell)> = HashSet::new();
        // erase boxes
        for pos in boxes.iter() {
            let cell_ref = map.get_cell_mut(*pos).unwrap();
            moved_boxes.insert(((*pos), *cell_ref));
            *cell_ref = Cell::Empty;
        }
        // put back moved boxes
        for (pos, cell) in moved_boxes {
            let target = get_moved_position(pos, map.get_size(), dir).unwrap();
            *map.get_cell_mut(target).unwrap() = cell;
        } 
        map.bot = get_moved_position(map.bot, map.get_size(), dir).unwrap();
    }
}

fn get_checksum(map: &Map) -> usize {
    map.cells.iter()
        .enumerate()
        .flat_map(|(i,row)|
            row.iter()
                .enumerate()
                .filter(|(_,c)| **c == Cell::Box || **c == Cell::BoxLeft)
                .map(move |(j,_)| (i,j))
        ).map(|(i,j)| i*100+j)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_small() {
        let mut map = parse_map("########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########");
        let moves = parse_directions("<^^>>>vv<v>>v<<");

        println!("{}\n", map._repr());
        for m in moves {
            try_move(&mut map, m);
            println!("{}\n", map._repr());
        }

        let target_map = parse_map("########\n#....OO#\n##.....#\n#.....O#\n#.#O@..#\n#...O..#\n#...O..#\n########");
        assert_eq!(map, target_map);

        assert_eq!(get_checksum(&map), 2028)
    }

    #[test]
    fn example_big() {
        let mut map = parse_map("##########\n#..O..O.O#\n#......O.#\n#.OO..O.O#\n#..O@..O.#\n#O#..O...#\n#O..O..O.#\n#.OO.O.OO#\n#....O...#\n##########");
        let moves = parse_directions("<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^");

        let target_map = parse_map("##########\n#.O.O.OOO#\n#........#\n#OO......#\n#OO@.....#\n#O#.....O#\n#O.....OO#\n#O.....OO#\n#OO....OO#\n##########");

        for m in moves.clone() {
            try_move(&mut map, m);
            println!("{}\n", map._repr());
        }
        
        assert_eq!(map, target_map);
    }

    #[test]
    fn example_big_doubled() {
        let mut map = parse_map("##########\n#..O..O.O#\n#......O.#\n#.OO..O.O#\n#..O@..O.#\n#O#..O...#\n#O..O..O.#\n#.OO.O.OO#\n#....O...#\n##########");
        map = double_map(&map);
        let moves = parse_directions("<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^");

        for m in moves.clone() {
            try_move(&mut map, m);
            println!("{}\n", map._repr());
        }
        
        assert_eq!(get_checksum(&map), 9021);
    }
}