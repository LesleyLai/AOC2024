const INPUT_REGISTERS: [isize; 3] = [55593699, 0, 0];
const INPUT_INSTRUCTIONS: &[isize] = &[2, 4, 1, 3, 7, 5, 0, 3, 1, 5, 4, 4, 5, 5, 3, 0];

struct Machine<'a> {
    ip: usize,
    registers: [isize; 3],
    instructions: &'a [isize],
}

impl<'a> Machine<'a> {
    fn new(registers: [isize; 3], instructions: &'a [isize]) -> Self {
        Self {
            ip: 0,
            registers,
            instructions,
        }
    }

    fn combo(&self) -> isize {
        let operand = self.instructions[self.ip + 1];
        match operand {
            0..=3 => operand,
            4..=6 => self.registers[(operand - 4) as usize],
            _ => unreachable!("Invalid operand: {}", operand),
        }
    }

    // return true if need continue
    fn step(&mut self) -> Option<isize> {
        let mut output = None;
        match self.instructions[self.ip] {
            0 => {
                self.registers[0] >>= self.combo();
            }
            1 => {
                self.registers[1] ^= self.instructions[self.ip + 1];
            }
            2 => {
                self.registers[1] = self.combo() % 8;
            }
            3 => {
                if self.registers[0] != 0 {
                    self.ip = self.instructions[self.ip + 1] as usize;
                    return None;
                }
            }
            4 => {
                self.registers[1] ^= self.registers[2];
            }
            5 => {
                output = Some(self.combo() % 8);
            }
            6 => {
                self.registers[1] = self.registers[0] >> self.combo();
            }
            7 => {
                self.registers[2] = self.registers[0] >> self.combo();
            }
            _ => unreachable!(),
        }
        self.ip += 2;
        output
    }
}

fn part1(registers: [isize; 3], instructions: &[isize]) -> Vec<isize> {
    let mut outputs = Vec::new();

    let mut machine = Machine::new(registers, instructions);
    while machine.ip < instructions.len() - 1 {
        if let Some(output) = machine.step() {
            outputs.push(output);
        }
    }
    outputs
}

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

fn assert_assumptions(instructions: &[isize]) {
    assert!(instructions.len() >= 6);

    // My assumptions to the input
    // There's only one instruction changes a into a >> 3
    // There’s a single jump at the end, which loops back to the beginning.
    // There is only one output instruction, which appears just before the jump at the end of the loop.
    // The states of b and c are irrelevant (they act like local variables within the loop). (untested)

    let mut meet_adv = false;
    for i in (0..instructions.len() - 4).step_by(2) {
        let instruction = instructions[i];

        assert_ne!(instruction, 3); // Not jump
        assert_ne!(instruction, 5); // Not out

        // a >>= 3
        if instruction == 0 {
            assert_eq!(instructions[i + 1], 3);
            assert!(!meet_adv);
            meet_adv = true;
        }
    }

    // single output instruction that output a register
    assert_eq!(instructions[instructions.len() - 4], 5);
    assert!(matches!(instructions[instructions.len() - 3], 4..6));

    // single loop instruction that jump back to beginning
    assert_eq!(instructions[instructions.len() - 2], 3);
    assert_eq!(instructions[instructions.len() - 1], 0);
}

// Gets all possible values of a
fn part2_inverse_step(
    a_after: isize,
    output: isize,
    instructions: &[isize],
) -> impl Iterator<Item = isize> + use<'_> {
    let a_before_min = a_after << 3;
    (a_before_min..(a_before_min + 8)).filter(move |&a| {
        let mut machine = Machine::new([a, 0, 0], instructions);
        while machine.ip < instructions.len() - 4 {
            machine.step();
        }
        let actual_output = machine.step().unwrap();
        let actual_a_after = machine.registers[0];

        (actual_a_after, actual_output) == (a_after, output)
    })
}

// A hopefully generic solution
fn part2(instructions: &[isize]) -> isize {
    assert_assumptions(instructions);

    // all potential a
    let mut candidates = vec![0];

    for &instruction in instructions.iter().rev() {
        let mut next_candidates = vec![];
        for &candidate_a in &candidates {
            next_candidates.extend(part2_inverse_step(candidate_a, instruction, instructions));
        }
        candidates = next_candidates;
    }

    *candidates.iter().min().unwrap()
}

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

    const TEST_INSTRUCTIONS: &[isize] = &[0, 3, 5, 4, 3, 0];

    // part 2
    // hardcoded test-case as rust code
    assert_eq!(test(2024), [5, 7, 3, 0]);
    assert_eq!(test(117440), TEST_INSTRUCTIONS);
    assert_eq!(test_inverse_run(TEST_INSTRUCTIONS), 117440);

    // hardcoded my input as rust code
    assert_eq!(input_program(55593699), [6, 0, 6, 3, 0, 2, 3, 1, 6]);
    assert_eq!(input_program(236539226447469), INPUT_INSTRUCTIONS);
    assert_eq!(
        input_program_inverse_run(INPUT_INSTRUCTIONS),
        236539226447469
    );

    // generic solution
    assert_eq!(part2(TEST_INSTRUCTIONS), 117440);
    assert_eq!(part2(INPUT_INSTRUCTIONS), 236539226447469);
}
