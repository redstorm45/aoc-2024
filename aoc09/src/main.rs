use std::collections::VecDeque;
use std::fs;
use std::env;


#[derive(PartialEq, Debug)]
struct FileInfo {
    id: usize,
    position: usize,
    size: usize
}

impl FileInfo {
    fn get_end(&self) -> usize { self.position + self.size }
    fn split(&self, back_size: usize) -> (FileInfo, FileInfo) {
        // Split this into a front and a back part each with same id
        (FileInfo{
            id: self.id,
            position: self.position,
            size: self.size - back_size
        }, FileInfo{
            id: self.id,
            position: self.position + self.size - back_size,
            size: back_size
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let disk = parse_disk_description(&contents);
    let merged_disk = merge_blocks(disk);
    let checksum = compute_checksum(&merged_disk);

    println!("Result: {}", checksum);
}

fn parse_disk_description(txt: &str) -> Vec<FileInfo> {
    let mut position: usize = 0;
    let mut res = vec![];
    for (i,c) in txt.chars().enumerate() {
        if c == '\n' {
            break;
        }
        let length =  c.to_digit(10).unwrap() as usize;
        if length == 0 {
            continue;
        }
        if i%2 == 1 {
            // empty
            position += length;
        } else {
            res.push(FileInfo{
                id: i/2,
                position,
                size: length
            });
            position += length;
        }
    }
    res
}

fn merge_blocks(source: Vec<FileInfo>) -> Vec<FileInfo> {
    if source.len() < 2 {
        return source;
    }
    let mut src = VecDeque::from(source);
    let mut res: Vec<FileInfo> = vec![];
    let mut back_block = src.pop_back().unwrap();
    let mut last_position = 0; // starting index of next insertion
    while !src.is_empty() {
        let target_position = src.front().unwrap().position;
        let space = target_position - last_position;
        if space == 0 {
            // insert start block
            let inserted_block = src.pop_front().unwrap();
            last_position += inserted_block.size;
            res.push(inserted_block);
        } else if back_block.size <= space {
            // nsert back block
            back_block.position = last_position;
            last_position += back_block.size;
            res.push(back_block);
            back_block = src.pop_back().unwrap();
        } else {
            // insert part of back block
            let (back_start, mut back_end) = back_block.split(space.min(back_block.size));
            back_block = back_start;
            back_end.position = last_position;
            last_position += back_end.size;
            res.push(back_end);

        }
    }
    // insert back block
    back_block.position = last_position;
    res.push(back_block);
    res
}

fn move_blocks(source: Vec<FileInfo>) -> Vec<FileInfo> {
    
}

fn compute_checksum(src: &[FileInfo]) -> usize {
    src.iter().map(|f| -> usize{
        f.id* (f.position..(f.position+f.size)).sum::<usize>()
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_simple() {
        assert_eq!(merge_blocks(parse_disk_description("12345")),
                   vec![
                       FileInfo{id: 0, position: 0, size: 1},
                       FileInfo{id: 2, position: 1, size: 2},
                       FileInfo{id: 1, position: 3, size: 3},
                       FileInfo{id: 2, position: 6, size: 3},
                   ]);
    }

    #[test]
    fn example_long() {
        // 00 99 8 111 888 2 777 333 6 44 6 5555 66
        assert_eq!(merge_blocks(parse_disk_description("2333133121414131402")),
                   vec![
                       FileInfo{id: 0, position: 0, size: 2},
                       FileInfo{id: 9, position: 2, size: 2},
                       FileInfo{id: 8, position: 4, size: 1},
                       FileInfo{id: 1, position: 5, size: 3},
                       FileInfo{id: 8, position: 8, size: 3},
                       FileInfo{id: 2, position: 11, size: 1},
                       FileInfo{id: 7, position: 12, size: 3},
                       FileInfo{id: 3, position: 15, size: 3},
                       FileInfo{id: 6, position: 18, size: 1},
                       FileInfo{id: 4, position: 19, size: 2},
                       FileInfo{id: 6, position: 21, size: 1},
                       FileInfo{id: 5, position: 22, size: 4},
                       FileInfo{id: 6, position: 26, size: 2},
                   ]);
    }

    #[test]
    fn example_long_sum() {
        assert_eq!(compute_checksum(&merge_blocks(parse_disk_description("2333133121414131402"))), 1928);
    }
}
