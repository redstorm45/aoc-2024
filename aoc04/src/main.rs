use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let grid: Vec<Vec<char>> = contents.split('\n')
                       .filter(|s| !s.is_empty())
                       .map(|s| s.chars().collect())
                       .collect();

    let mut count1 = count_xmas(&grid);
    let mut count2 = count_masmas(&grid);

    println!("Result: {}", count1);
    println!("Result2: {}", count2);
}

fn count_xmas(grid: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    count += pattern_lookup(grid, &vec![vec![Some('X'), Some('M'), Some('A'), Some('S')]]).len();
    count += pattern_lookup(grid, &vec![vec![Some('S'), Some('A'), Some('M'), Some('X')]]).len();
    count += pattern_lookup(grid, &vec![vec![Some('X')], vec![Some('M')], vec![Some('A')], vec![Some('S')]]).len();
    count += pattern_lookup(grid, &vec![vec![Some('S')], vec![Some('A')], vec![Some('M')], vec![Some('X')]]).len();

    count += pattern_lookup(grid, &vec![vec![Some('X'), None, None, None],
                                           vec![None, Some('M'), None, None],
                                           vec![None, None, Some('A'), None],
                                           vec![None, None, None, Some('S')]]).len();
    count += pattern_lookup(grid, &vec![vec![Some('S'), None, None, None],
                                            vec![None, Some('A'), None, None],
                                            vec![None, None, Some('M'), None],
                                            vec![None, None, None, Some('X')]]).len();
    count += pattern_lookup(grid, &vec![vec![None, None, None, Some('X')],
                                            vec![None, None, Some('M'), None],
                                            vec![None, Some('A'), None, None],
                                            vec![Some('S'), None, None, None]]).len();
    count += pattern_lookup(grid, &vec![vec![None, None, None, Some('S')],
                                            vec![None, None, Some('A'), None],
                                            vec![None, Some('M'), None, None],
                                            vec![Some('X'), None, None, None]]).len();
    return count;
}

fn count_masmas(grid: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    count += pattern_lookup(grid, &vec![vec![Some('M'), None, Some('S')],
                                           vec![None, Some('A'), None],
                                           vec![Some('M'), None, Some('S')]]).len();
    count += pattern_lookup(grid, &vec![vec![Some('M'), None, Some('M')],
                                            vec![None, Some('A'), None],
                                            vec![Some('S'), None, Some('S')]]).len();
    count += pattern_lookup(grid, &vec![vec![Some('S'), None, Some('S')],
                                            vec![None, Some('A'), None],
                                            vec![Some('M'), None, Some('M')]]).len();
    count += pattern_lookup(grid, &vec![vec![Some('S'), None, Some('M')],
                                            vec![None, Some('A'), None],
                                            vec![Some('S'), None, Some('M')]]).len();

                                            /*
    count += pattern_lookup(grid, &vec![vec![None, Some('S'), None],
                                            vec![Some('S'), Some('A'), Some('M')],
                                            vec![None, Some('M'), None]]).len();
    count += pattern_lookup(grid, &vec![vec![None, Some('S'), None],
                                            vec![Some('M'), Some('A'), Some('S')],
                                            vec![None, Some('M'), None]]).len();
    count += pattern_lookup(grid, &vec![vec![None, Some('M'), None],
                                            vec![Some('S'), Some('A'), Some('M')],
                                            vec![None, Some('S'), None]]).len();
    count += pattern_lookup(grid, &vec![vec![None, Some('M'), None],
                                            vec![Some('M'), Some('A'), Some('S')],
                                            vec![None, Some('S'), None]]).len();
                                            */

    return count;
}

fn pattern_lookup(grid: &Vec<Vec<char>>, pat: &Vec<Vec<Option<char>>>) -> Vec<(usize,usize)>{
    let height = grid.len();
    let width = grid.first().unwrap().len();
    let pat_height = pat.len();
    let pat_width = pat.first().unwrap().len();

    let mut res = vec![];

    for di in 0..(height - pat_height+1) {
        for dj in 0..(width - pat_width+1) {
            let mut pat_match = true;
            for i in 0..pat_height {
                for j in 0..pat_width {
                    if let Some(val) = pat[i][j] {
                        pat_match = val == grid[di+i][dj+j];
                    } else {
                        continue;
                    }
                    if !pat_match {
                        break;
                    }
                }
                if !pat_match {
                    break;
                }
            }
            if pat_match {
                res.push((di, dj));
            }
        }
    }

    return res;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_xmas_example() {
        let grid = "....XXMAS.\n.SAMXMS...\n...S..A...\n..A.A.MS.X\nXMASAMX.MM\nX.....XA.A\nS.S.S.S.SS\n.A.A.A.A.A\n..M.M.M.MM\n.X.X.XMASX"
            .split('\n')
            .map(|s| s.chars().collect())
            .collect();
        assert_eq!(count_xmas(&grid), 18);
    }

    #[test]
    fn count_masmas_example() {
        let grid = ".M.S......\n..A..MSMS.\n.M.S.MAA..\n..A.ASMSM.\n.M.S.M....\n..........\nS.S.S.S.S.\n.A.A.A.A..\nM.M.M.M.M.\n.........."
            .split('\n')
            .map(|s| s.chars().collect())
            .collect();
        assert_eq!(count_masmas(&grid), 9);
    }

    
}

