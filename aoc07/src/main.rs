use std::fs;
use std::env;


enum Operator {
    Add,
    Multiply,
    Concat
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let equations: Vec<(i128,Vec<i128>)> = contents.split_terminator('\n')
        .map(|s| s.split_once(": ").unwrap())
        .map(|(k,v)| (
            k.parse::<i128>().unwrap(),
            v.split(' ').map(|e| e.parse::<i128>().unwrap()).collect()
        ))
        .collect();

    let (valid_eqs, invalid_eqs): (Vec<(i128,Vec<i128>)>,Vec<(i128,Vec<i128>)>) = equations.iter()
        .cloned()
        .partition(|(r, ops)| is_valid_equation(*r, ops, &[Operator::Add, Operator::Multiply]));

    let valid_std: i128 = valid_eqs
        .iter()
        .map(|(r,_)| r)
        .sum();

    let more_valid: i128 = invalid_eqs
        .iter()
        .filter(|(r, ops)| is_valid_equation(*r, ops, &[Operator::Add, Operator::Multiply, Operator::Concat]))
        .map(|(r,_)| r)
        .sum();
    let valid_comp = valid_std + more_valid;

    println!("Result: {}", valid_std);
    println!("Result2: {}", valid_comp);
}

fn is_valid_equation(result: i128, operands: &[i128], operators: &[Operator]) -> bool {
    is_valid_equation_completion(result, &operands[1..], operators, operands[0])
}

fn op_concat(a: i128, b: i128) -> Option<i128> {
    let size = ((b+1) as f64).log10().ceil() as u32;
    10_i128.checked_pow(size).and_then(|m| a.checked_mul(m)).map(|am| am+b)
}

fn is_valid_equation_completion(result: i128, operands: &[i128], operators: &[Operator], current_result: i128) -> bool {
    if current_result > result {
        false
    } else if operands.is_empty() {
        result == current_result
    } else {
        return operators.iter().any(|op| 
            match *op {
                Operator::Add => is_valid_equation_completion(result, &operands[1..], operators, current_result+operands[0]),
                Operator::Multiply => {
                    if let Some(multiplied) = current_result.checked_mul(operands[0]) {
                        is_valid_equation_completion(result, &operands[1..], operators, multiplied)
                    } else {
                        false
                    }
                },
                Operator::Concat => {
                    if let Some(concatenated) = op_concat(current_result,operands[0]) {
                        is_valid_equation_completion(result, &operands[1..], operators, concatenated)
                    } else {
                        false
                    }
                },
            }
        );
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat_tests() {
        assert_eq!(op_concat(1, 1), Some(11));
        assert_eq!(op_concat(9, 1), Some(91));
        assert_eq!(op_concat(111, 111), Some(111_111));
        assert_eq!(op_concat(999, 999), Some(999_999));
    }
}
