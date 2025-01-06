use std::collections::VecDeque;
use std::fs;
use std::env;


#[derive(PartialEq, Debug, Clone)]
struct FileInfo {
    id: usize,
    position: usize,
    size: usize
}

impl FileInfo {
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
    let merged_disk = merge_blocks(disk.clone());
    let checksum1 = compute_checksum(&merged_disk);
    let moved_disk = move_blocks(disk);
    let checksum2 = compute_checksum(&moved_disk);

    println!("Result: {}", checksum1);
    println!("Result: {}", checksum2);
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
            // insert back block
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

fn insert_block(disk: &mut Vec<FileInfo>, to_add: FileInfo) {
    let res = disk.binary_search_by_key(&to_add.position, |f| f.position);
    let first_greatest = res.expect_err("Duplicate index");
    disk.insert(first_greatest, to_add);
}

fn move_blocks(source: Vec<FileInfo>) -> Vec<FileInfo> {
    let mut holes_by_size : Vec<Vec<FileInfo>> = vec![];

    let mut position = 0;
    for block in source.iter() {
        if block.position > position {
            let size = block.position - position;
            while holes_by_size.len() <= size {
                holes_by_size.push(vec![]);
            }
            holes_by_size.get_mut(size).unwrap().push(FileInfo{
                id: 0,
                position,
                size: block.position - position
            });
        }
        position = block.position + block.size;
    }

    let mut res = vec![];
    for src_block in source.iter().rev() {
        let mut inserted = false;
        if src_block.size < holes_by_size.len() {
            let mut best_hole: Option<&FileInfo> = None;
            for hole_list in &holes_by_size[src_block.size..] {
                // only the first hole is interesting (if it exists)
                if let Some(first_hole) = hole_list.first() {
                    // hole after current or worse than best is useless
                    if first_hole.position < src_block.position && best_hole.is_some_and(|b| b.position > first_hole.position){
                        best_hole = Some(first_hole);
                    }
                }
            }
            if let Some(target_hole) = best_hole.cloned() {
                // remove hole from list
                let target_hole_list = holes_by_size.get_mut(target_hole.size).unwrap();
                let index = target_hole_list.iter().position(|h| h.position==target_hole.position).unwrap();
                target_hole_list.remove(index);
                // create a new hole
                let new_hole = FileInfo{
                    position: target_hole.position + src_block.size,
                    id: 0,
                    size: target_hole.size - src_block.size
                };
                // insert the new hole
                let new_hole_list = holes_by_size.get_mut(new_hole.size).unwrap();
                let index_new = new_hole_list.binary_search_by_key(&new_hole.position, |h| h.position).expect_err("Duplicate hole index");
                new_hole_list.insert(index_new, new_hole);
                // shorten hole list if needed
                if target_hole.size == holes_by_size.len() {
                    while !holes_by_size.is_empty() && holes_by_size.last().unwrap().is_empty() {
                        holes_by_size.pop();
                    }
                }
                // insert the block
                let new_block = FileInfo{
                    position: target_hole.position,
                    id: src_block.id,
                    size: src_block.size
                };
                insert_block(&mut res, new_block);
                inserted = true;
            }
        }
        if !inserted {
            insert_block(&mut res, src_block.clone());
        }
    }
    res
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
