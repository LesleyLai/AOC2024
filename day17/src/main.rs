const INPUT_REGISTERS: [isize; 3] = [55593699, 0, 0];
const INPUT_INSTRUCTIONS: &[isize] = &[2, 4, 1, 3, 7, 5, 0, 3, 1, 5, 4, 4, 5, 5, 3, 0];

fn combo_operand_value(ip: usize, instructions: &[isize], registers: [isize; 3]) -> isize {
    let operand = instructions[ip + 1];
    if operand < 4 {
        operand as isize
    } else if operand < 7 {
        registers[(operand - 4) as usize]
    } else {
        unreachable!()
    }
}

fn part1(mut registers: [isize; 3], instructions: &[isize]) -> Vec<isize> {
    let mut output = Vec::new();

    let mut ip = 0;

    while ip < instructions.len() - 1 {
        match instructions[ip] {
            0 => {
                registers[0] = registers[0] >> combo_operand_value(ip, instructions, registers);
            }
            1 => {
                registers[1] ^= instructions[ip + 1] as isize;
            }
            2 => {
                registers[1] = combo_operand_value(ip, instructions, registers) % 8;
            }
            3 => {
                if registers[0] != 0 {
                    ip = instructions[ip + 1] as usize;
                    continue;
                }
            }
            4 => {
                registers[1] ^= registers[2];
            }
            5 => {
                output.push(combo_operand_value(ip, instructions, registers) % 8);
            }
            6 => {
                registers[1] = registers[0] >> combo_operand_value(ip, instructions, registers);
            }
            7 => {
                registers[2] = registers[0] >> combo_operand_value(ip, instructions, registers);
            }
            _ => unreachable!(),
        }
        ip += 2;
    }
    output
}

// fn assert_assumptions(instructions: &[isize]) {
//     assert!(instructions.len() >= 4);
//
//     // We only deal with the situation of a single jump that jump back to beginning
//     for i in (0..instructions.len() - 2).step_by(2) {
//         assert_ne!(instructions[i], 3); // No jump instruction
//     }
//     assert_eq!(instructions[instructions.len() - 2], 3); // jump at end
//     assert_eq!(instructions[instructions.len() - 1], 0); // jump back to beginning
//
//     // Only Single out instruction in a loop that output a register value
//     let mut has_out_instr = false;
//     for i in (0..instructions.len() - 2).step_by(2) {
//         if instructions[i] == 5 {
//             assert!(!has_out_instr);
//             has_out_instr = true;
//             assert!(instructions[i + 1] >= 4);
//             assert!(instructions[i + 1] < 7);
//         }
//     }
// }

fn test(mut a: isize) -> Vec<isize> {
    let mut result = vec![];

    loop {
        let (a_after, output) = test_input_step(a);
        result.push(output);
        a = a_after;
        if a == 0 {
            break;
        }
    }
    result
}

fn test_input_step(mut a: isize) -> (isize, isize) {
    a >>= 3;
    (a, a % 8)
}

fn test_inverse_run(instructions: &[isize]) -> isize {
    let mut a = 0;
    for &instruction in instructions.iter().rev() {
        a += instruction;
        a <<= 3;
    }
    a
}

fn input_program_step(mut a: isize) -> (isize, isize) {
    let mut b = a % 8;
    b ^= 3;
    let c = a >> b;
    a >>= 3;
    b ^= 5;
    b ^= c;

    (a, b % 8)
}

fn input_program(mut a: isize) -> Vec<isize> {
    let mut result = Vec::new();
    loop {
        let (a_after, output) = input_program_step(a);
        a = a_after;

        result.push(output);

        if a == 0 {
            break;
        }
    }
    result
}

// Gets all possible values of a
fn input_program_inverse_step(a_after: isize, b_after: isize) -> Vec<isize> {
    let mut candidates = vec![];

    let a_before_min = a_after << 3;
    for a in a_before_min..(a_before_min + 8) {
        if input_program_step(a) == (a_after, b_after) {
            candidates.push(a);
        }
    }

    candidates
}

fn input_program_inverse_run(instructions: &[isize]) -> isize {
    // potential a s
    let mut candidates = vec![0];

    for &instruction in instructions.iter().rev() {
        let mut next_candidates = vec![];
        for &candidate_a in &candidates {
            next_candidates.extend(input_program_inverse_step(candidate_a, instruction));
        }
        candidates = next_candidates;
    }

    *candidates.iter().min().unwrap()
}

// fn part2(instructions: &[isize]) {
//     assert_assumptions(instructions);
//
//     let mut output_register = None;
//     for i in (0..instructions.len() - 2).step_by(2) {
//         if instructions[i] == 5 {
//             output_register = Some(usize::try_from(instructions[i + 1] - 4).unwrap());
//         }
//     }
//     let output_register = output_register.unwrap();
//
//     println!("output_register: {}", output_register);
//
//     // a most be 0 at the end
//
//     //
//     let mut ip = instructions.len() - 4;
//     for output in instructions.iter().rev() {
//         println!("output: {}", output);
//     }
// }

#[allow(dead_code)]
fn part2_brute(instructions: &[isize]) {
    let mut i = 0;
    loop {
        if input_program(i) == instructions {
            println!("Get result: {}", i);
            return;
        }

        if i % 32768 == 0 {
            println!("Iteration {}", i);
        }
        i += 1;
    }
}

fn main() {
    assert_eq!(
        part1([729, 0, 0], &[0, 1, 5, 4, 3, 0]),
        [4, 6, 3, 5, 6, 3, 5, 2, 1, 0]
    );

    assert_eq!(
        part1(INPUT_REGISTERS, INPUT_INSTRUCTIONS),
        [6, 0, 6, 3, 0, 2, 3, 1, 6]
    );

    assert_eq!(
        part1([117440, 0, 0], &[0, 3, 5, 4, 3, 0]),
        [0, 3, 5, 4, 3, 0]
    );

    // part 2
    // hardcoded test-case as rust code
    assert_eq!(test(2024), [5, 7, 3, 0]);
    assert_eq!(test(117440), [0, 3, 5, 4, 3, 0]);
    assert_eq!(test_inverse_run(&[0, 3, 5, 4, 3, 0]), 117440);

    // hardcoded my input as rust code
    assert_eq!(input_program(55593699), [6, 0, 6, 3, 0, 2, 3, 1, 6]);

    assert_eq!(
        input_program_inverse_run(INPUT_INSTRUCTIONS),
        236539226447469
    );
}
