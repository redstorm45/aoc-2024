use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let mut count_safe = 0;
    let mut count_safe_ignoring = 0;
    for line in contents.split('\n').filter(|s| s.len()>0) {
        let report = line.split(' ')
                         .map(|v| v.parse::<i32>().unwrap())
                         .collect::<Vec<i32>>();
        if is_report_safe(&report) {
            count_safe += 1;
            count_safe_ignoring += 1;
        } else if is_report_safe_ignoring(report) {
            count_safe_ignoring += 1;
        }
    }
    println!("Result: {}", count_safe);
    println!("Result2: {}", count_safe_ignoring);
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    if report.len() <= 1 {
        return true;
    }
    let increasing: bool = report.first().unwrap() < report.get(1).unwrap();

    for i in 0..(report.len()-1){
        let (cur, next) = (report.get(i).unwrap(), report.get(i+1).unwrap());
        if cur == next {
            return false;
        } else if increasing && cur > next {
            return false;
        } else if !increasing && cur < next {
            return false;
        } else if (next-cur).abs() > 3 {
            return false;
        }
    }
    return true;
}

fn is_report_safe_ignoring(report: Vec<i32>) -> bool {
    /*
    Is a report safe, if one of the values can be removed
    */
    if is_report_safe(&report) {
        return true;
    }
    // brute-force
    for i in 0..report.len() {
        let split_report: Vec<i32> = report.iter().enumerate().filter(|(j,_)| *j!=i).map(|(_,&e)| e).collect();
        if is_report_safe(&split_report) {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_examples() {
        assert_eq!(is_report_safe(&vec![7,6,4,2,1]), true);
        assert_eq!(is_report_safe(&vec![1,2,7,8,9]), false);
        assert_eq!(is_report_safe(&vec![9,7,6,2,1]), false);
        assert_eq!(is_report_safe(&vec![1,3,2,4,5]), false);
        assert_eq!(is_report_safe(&vec![8,6,4,4,1]), false);
        assert_eq!(is_report_safe(&vec![1,3,6,7,9]), true);
    }
}

