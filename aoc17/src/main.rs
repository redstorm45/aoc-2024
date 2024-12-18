
use core::panic;
use std::fs;
use std::env;

/*
First program:

2,4  0:  a%8 -> b
1,1  2:  b^1 -> b
7,5  4:  a>>b -> c
1,4  6:  b^4 -> b
0,3  8:  a>>3 -> a
4,5  10: b^c -> b
5,5  12: out b%8
3,0  14: if(a!=0) goto 0


reverse
 if must have failed => a = 0
 out b%8   => b = 0
 b^c -> b  => b^c = 0
 a>>3 -> a => a in [0..7]
 b^4 -> b  => (b^4)^c = 0
 a>>b -> c => [explicit test all a] -> (a,b,c) in [..]
 b^1 -> b  => (a,b,c) in [..]
 a%8 -> b  => (a,b,c) in [..]
 program must have looped -> a!=0
 */


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let mut splitter = contents.split_terminator('\n');
    let reg_a: MachineUnit = splitter.next().unwrap().split(": ").nth(1).unwrap().parse().unwrap();
    let reg_b: MachineUnit = splitter.next().unwrap().split(": ").nth(1).unwrap().parse().unwrap();
    let reg_c: MachineUnit = splitter.next().unwrap().split(": ").nth(1).unwrap().parse().unwrap();
    let instructions: Vec<i8> = splitter.nth(1).unwrap().split(": ").nth(1).unwrap().split(',').map(|e| e.parse::<i8>().unwrap()).collect();

    let mut state = MachineState{
        register_a: reg_a,
        register_b: reg_b,
        register_c: reg_c,
        program_counter: 0,
    };

    //println!("Running with state {:?} and instructions {:?}", state, instructions);

    let output = run_until_halt(&mut state, &instructions);

    let iter1 = output.iter().map(|v| v.to_string());
    let output_str = itertools::intersperse(iter1, String::from(",")).fold(String::new(), |cur, nxt| cur + &nxt);
    println!("Result: {}", output_str);

    /*
    let mut found = false;
    for i in 5..1000000000 {
        let mut state2 = MachineState{
            register_a: i,
            register_b: 0,
            register_c: 0,
            program_counter: 0,
        };
        let output = run_until_halt(&mut state2, &instructions);
        if output == instructions {
            println!("Found machine: {}", i);
            break;
        }
    }
    if !found {
        println!("Machine not found");
    }
    */

    let rev = revert_program(&instructions);
    println!("Result2: {}", rev);

    /*
    let mut state = MachineState{
        register_a: 202322936867370,
        register_b: 0,
        register_c: 0,
        program_counter: 0,
    };

    let output2 = run_until_halt(&mut state, &instructions);
    let output2_str = output2.iter().map(|v| v.to_string()).intersperse(String::from(",")).fold(String::new(), |cur, nxt| cur + &nxt);
    println!("New result: {}", output2_str);
    */
}

type MachineUnit = usize;

#[derive(Debug)]
struct MachineState {
    register_a: MachineUnit,
    register_b: MachineUnit,
    register_c: MachineUnit,
    program_counter: usize
}

fn read_combo(state: &mut MachineState, instructions: &[i8], index: usize) -> MachineUnit {
    let target = instructions[index];
    match target {
        0..=3 => target as MachineUnit,
        4 => state.register_a,
        5 => state.register_b,
        6 => state.register_c,
        _ => panic!("Unhandled combo value")
    }
}

fn run_one_step(state: &mut MachineState, instructions: &[i8]) -> (Option<i8>,bool) {
    if state.program_counter >= instructions.len() {
        return (None, false);
    }
    match instructions[state.program_counter] {
        0 => { // adv
            let numerator = state.register_a;
            let denominator_power = read_combo(state, instructions, state.program_counter+1);
            let result = numerator >> denominator_power;
            state.register_a = result;
            state.program_counter += 2;
        },
        1 => { // bxl
            let operand = instructions[state.program_counter+1];
            state.register_b = state.register_b ^ (operand as MachineUnit);
            state.program_counter += 2;
        },
        2 => { // bst
            let operand = read_combo(state, instructions, state.program_counter+1);
            let value = operand % 8;
            state.register_b = value;
            state.program_counter += 2;
        },
        3 => { // jnz
            if state.register_a == 0 {
                state.program_counter += 2;
            } else {
                state.program_counter = instructions[state.program_counter+1] as usize;
            }
        },
        4 => { // bxc
            state.register_b = state.register_b ^ state.register_c;
            state.program_counter += 2;
        },
        5 => { // out
            let operand = read_combo(state, instructions, state.program_counter+1);
            let value = (operand % 8) as i8;
            state.program_counter += 2;
            return (Some(value), true);
        },
        6 => { // bdv
            let numerator = state.register_a;
            let denominator_power = read_combo(state, instructions, state.program_counter+1);
            let result = numerator >> denominator_power;
            state.register_b = result;
            state.program_counter += 2;
        },
        7 => { // cdv
            let numerator = state.register_a;
            let denominator_power = read_combo(state, instructions, state.program_counter+1);
            let result = numerator >> denominator_power;
            state.register_c = result;
            state.program_counter += 2;
        }
        _ => {panic!("Unhandled instruction")}
    }
    (None, true)
}

fn run_until_halt(state: &mut MachineState, instructions: &[i8]) -> Vec<i8> {
    let mut running = true;
    let mut res = vec![];
    while running {
        let step_result = run_one_step(state, instructions);
        running = step_result.1;
        if let Some(o) = step_result.0 {
            res.push(o);
        }
    }
    res
}

fn revert_program(target_output: &[i8]) -> usize {
    // hardcoded algorithm for this program
    // every step formula is:
    // a_ ^ 5 ^ ((a>>(a_^1)) %8) = out
    // with a the full reg_a value, and a_ the lowest 3 bits
    // before first loop, a=0
    // before formula, a<<3 -> every step, try all a_

    let mut values: Vec<usize> = vec![0];

    for output in target_output.iter().rev() {
        //println!("Revert from value {:?} with output {}", values, output);
        let mut new_values = vec![];
        for v in values {
            for i in 0..=7 {
                let a = v*8 +i;
                if i ^ 5 ^ ((a>>(i^1))%8) == *output as usize {
                    new_values.push(a);
                }
            }
        }
        values = new_values;
    }

    values.sort();

    *values.first().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_unit_1() {
        let mut machine = MachineState{
            register_a: 0,
            register_b: 0,
            register_c: 9,
            program_counter: 0
        };

        let output = run_until_halt(&mut machine, &[2, 6]);
        assert_eq!(output, vec![]);
        assert_eq!(machine.register_b, 1);
    }

    #[test]
    fn example_unit_2() {
        let mut machine = MachineState{
            register_a: 10,
            register_b: 0,
            register_c: 0,
            program_counter: 0
        };

        let output = run_until_halt(&mut machine, &[5,0,5,1,5,4]);
        assert_eq!(output, vec![0,1,2]);
    }

    #[test]
    fn example_unit_3() {
        let mut machine = MachineState{
            register_a: 2024,
            register_b: 0,
            register_c: 0,
            program_counter: 0
        };

        let output = run_until_halt(&mut machine, &[0,1,5,4,3,0]);
        assert_eq!(output, vec![4,2,5,6,7,7,7,7,3,1,0]);
        assert_eq!(machine.register_a, 0);
    }

    #[test]
    fn example_unit_4() {
        let mut machine = MachineState{
            register_a: 0,
            register_b: 29,
            register_c: 0,
            program_counter: 0
        };

        let output = run_until_halt(&mut machine, &[1,7]);
        assert_eq!(output, vec![]);
        assert_eq!(machine.register_b, 26);
    }

    #[test]
    fn example_unit_5() {
        let mut machine = MachineState{
            register_a: 0,
            register_b: 2024,
            register_c: 43690,
            program_counter: 0
        };

        let output = run_until_halt(&mut machine, &[4,0]);
        assert_eq!(output, vec![]);
        assert_eq!(machine.register_b, 44354);
    }

    #[test]
    fn example_unit_div() {
        let mut machine = MachineState{
            register_a: 12,
            register_b: 1,
            register_c: 0,
            program_counter: 0
        };

        let output = run_until_halt(&mut machine, &[0,5]);
        assert_eq!(output, vec![]);
        assert_eq!(machine.register_a, 6);
    }

    #[test]
    fn example() {
        let mut machine = MachineState{
            register_a: 729,
            register_b: 0,
            register_c: 0,
            program_counter: 0
        };

        let output = run_until_halt(&mut machine, &[0,1,5,4,3,0]);
        assert_eq!(output, vec![4,6,3,5,6,3,5,2,1,0]);
    }
}