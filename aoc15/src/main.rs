use std::fs;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let mut split_str = contents.split("\n\n");
    let mut map = parse_map(split_str.next().unwrap());
    let moves = parse_directions(split_str.next().unwrap());

    for m in moves {
        try_move(&mut map, m);
    }

    let gps_sum = get_checksum(&map);

    println!("Result: {}", gps_sum);
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Cell {
    Empty,
    Wall,
    Box,
    //BoxLeft,
    //BoxRight
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
    fn get_cell_mut(&mut self, pos: (usize, usize)) -> Option<&mut Cell> {
        self.cells.get_mut(pos.0).and_then(|row| row.get_mut(pos.1))
    }
    fn repr(&self) -> String {
        let mut res: String = String::new();
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                if self.bot == (i,j) {
                    res += "@";
                } else if self.cells[i][j] == Cell::Wall {
                    res += "#";
                } else if self.cells[i][j] == Cell::Box {
                    res += "O";
                } else {
                    res += ".";
                }
            }
            res += "\n";
        }
        res
    }
}

#[derive(Clone, Copy)]
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
/*
fn double_map(map: &Map) -> Map {

}
*/

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

fn try_move(map: &mut Map, dir: Direction) {
    let adj_pos = get_moved_position(map.bot, map.get_size(), dir);
    if adj_pos.is_none() {
        return;
    }
    {
        let adj_cell = map.get_cell_mut(adj_pos.unwrap()).unwrap();
        if *adj_cell == Cell::Wall {
            return;
        }
        if *adj_cell == Cell::Empty {
            map.bot = adj_pos.unwrap();
            return;
        }
    }
    // pushing a box
    let mut first_loop = true;
    let mut after_box = (adj_pos.unwrap(), Cell::Box);
    while first_loop || after_box.1==Cell::Box {
        first_loop = false;
        let next_pos = get_moved_position(after_box.0, map.get_size(), dir).unwrap();
        after_box = (next_pos, *map.get_cell_mut(next_pos).unwrap());
    }
    if after_box.1 == Cell::Wall {
        return;
    }
    *map.get_cell_mut(adj_pos.unwrap()).unwrap() = Cell::Empty;
    map.bot = adj_pos.unwrap();
    *map.get_cell_mut(after_box.0).unwrap() = Cell::Box;
}

fn get_checksum(map: &Map) -> usize {
    map.cells.iter()
        .enumerate()
        .flat_map(|(i,row)|
            row.iter()
                .enumerate()
                .filter(|(_,c)| **c == Cell::Box)
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

        println!("{}\n", map.repr());
        for m in moves {
            try_move(&mut map, m);
            println!("{}\n", map.repr());
        }

        let target_map = parse_map("########\n#....OO#\n##.....#\n#.....O#\n#.#O@..#\n#...O..#\n#...O..#\n########");
        assert_eq!(map, target_map);

        assert_eq!(get_checksum(&map), 2028)
    }
}