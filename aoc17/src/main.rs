
use core::panic;
use std::collections::VecDeque;
use std::fs;
use std::env;
use itertools::Itertools;

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
    let reg_a: i32 = splitter.next().unwrap().split(": ").nth(1).unwrap().parse().unwrap();
    let reg_b: i32 = splitter.next().unwrap().split(": ").nth(1).unwrap().parse().unwrap();
    let reg_c: i32 = splitter.next().unwrap().split(": ").nth(1).unwrap().parse().unwrap();
    let instructions: Vec<i8> = splitter.nth(1).unwrap().split(": ").nth(1).unwrap().split(',').map(|e| e.parse::<i8>().unwrap()).collect();

    let mut state = MachineState{
        register_a: reg_a,
        register_b: reg_b,
        register_c: reg_c,
        program_counter: 0,
    };

    //println!("Running with state {:?} and instructions {:?}", state, instructions);

    let output = run_until_halt(&mut state, &instructions);

    let output_str = output.iter().map(|v| v.to_string()).intersperse(String::from(",")).fold(String::new(), |cur, nxt| cur + &nxt);
    println!("Result: {}", output_str);

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
}

#[derive(Debug)]
struct MachineState {
    register_a: i32,
    register_b: i32,
    register_c: i32,
    program_counter: usize
}

fn read_combo(state: &mut MachineState, instructions: &[i8], index: usize) -> i32 {
    let target = instructions[index];
    match target {
        0..=3 => target as i32,
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
            state.register_b = state.register_b ^ (operand as i32);
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

#[derive(PartialEq, Eq)]
enum Register{
    A,
    B,
    C
}

// possible state of the program at an intermediate backtracking step
// register is None if any value works
struct BackTrackState {
    program_counter: usize, // program counter after executing previous instruction
    completed_output: usize, // number of outputs done up to this point (counting from end)
    register_a: Option<i32>,
    register_b: Option<i32>,
    register_c: Option<i32>,
    completed: bool, // start of the program has been reached
}

impl BackTrackState {
    fn with_program_counter_diff(&self, pc_diff: isize) -> BackTrackState {
        self.with_program_counter(((self.program_counter as isize)+pc_diff) as usize)
    }
    fn with_program_counter(&self, pc: usize) -> BackTrackState {
        BackTrackState{
            program_counter: pc,
            completed_output: self.completed_output,
            register_a: self.register_a,
            register_b: self.register_b,
            register_c: self.register_c,
            completed: self.completed
        }
    }
    fn with_completed(&self) -> BackTrackState {
        BackTrackState{
            program_counter: self.program_counter,
            completed_output: self.completed_output,
            register_a: self.register_a,
            register_b: self.register_b,
            register_c: self.register_c,
            completed: true
        }
    }
    fn with_register_a(&self, reg_a: Option<i32>) -> BackTrackState {
        BackTrackState{
            program_counter: self.program_counter,
            completed_output: self.completed_output,
            register_a: reg_a,
            register_b: self.register_b,
            register_c: self.register_c,
            completed: self.completed
        }
    }
    fn with_register_b(&self, reg_b: Option<i32>) -> BackTrackState {
        BackTrackState{
            program_counter: self.program_counter,
            completed_output: self.completed_output,
            register_a: self.register_a,
            register_b: reg_b,
            register_c: self.register_c,
            completed: self.completed
        }
    }
    fn with_register_c(&self, reg_c: Option<i32>) -> BackTrackState {
        BackTrackState{
            program_counter: self.program_counter,
            completed_output: self.completed_output,
            register_a: self.register_a,
            register_b: self.register_b,
            register_c: reg_c,
            completed: self.completed
        }
    }
    fn with_register(&self, reg: Register, value: Option<i32>) -> BackTrackState {
        match reg {
            Register::A => self.with_register_a(value),
            Register::B => self.with_register_b(value),
            Register::C => self.with_register_c(value),
        }
    }
    fn get_register(&self, reg: Register) -> Option<i32> {
        match reg {
            Register::A => self.register_a,
            Register::B => self.register_b,
            Register::C => self.register_c
        }
    }
}

fn propose_shifting_backtracks(state: &BackTrackState, current_instruction: i8, current_arg: i8) -> Vec<BackTrackState> {
    // group adv, bdc, cdv into one call
    // reverses instructions (a/b/c) = a>>(a/b/c/0..3)
    let target_register = match current_instruction {
        0 => Register::A,
        6 => Register::B,
        7 => Register::C,
        _ => panic!("Unsupported division")
    };
    match current_arg {
        0..=3 => {
            // fixed shift => remainder has been lost
            if let Some(reg) = state.get_register(target_register) {
                let mut res = vec![];
                for remainder in 0..=(2<<current_arg) {
                    res.push(
                        state.with_program_counter_diff(-2)
                             .with_register_a(Some((reg<<current_arg) + remainder))
                    );
                }
                res
            } else {
                vec![state.with_program_counter_diff(-2)]
            }
        },
        4..=6 => {
            // variable shift, 
        }
        _ => {panic!("Unsupported division");}
    }
}

fn propose_backtracks(state: &BackTrackState, next_output: Option<i8>, program: &[i8]) -> Vec<BackTrackState> {
    // propose backtracking options for one state
    let mut possibilities = vec![];
    // TODO: backtracking of jmp here
    // suppose there was no jmp

    if state.program_counter == 0 {
        possibilities.push(state.with_completed());
    } else {
        let prev_instruction = program[state.program_counter-2];
        let prev_arg = program[state.program_counter-1];

        match prev_instruction {
            0 => { // adv   a>>arg -> a
                possibilities.extend(propose_shifting_backtracks(state, prev_instruction, prev_arg));
            },
            _ => panic!("Unsupported instruction")
        }
    }

    possibilities
}

fn backtrack_all(initial: BackTrackState, target_output: &[i8], program: &[i8]) -> Vec<BackTrackState> {
    // backtrack all paths until start of the program
    // does a breadth-first traversal until all roots (backtrack at start with complete output) are found
    // and returns all those roots

    // TODO: parse jmp table first, [targetaddress -> jmp address]

    let mut to_explore : VecDeque<BackTrackState> = VecDeque::from([initial]);
    let mut roots: Vec<BackTrackState> = vec![];

    while to_explore.len() > 0 {
        let exploredState = to_explore.pop_front().unwrap();

    }

    roots
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