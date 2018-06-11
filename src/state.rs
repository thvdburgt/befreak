use std::fmt;

use direction::Direction;
use instruction::Instruction;
use program::Program;
use stack::Stack;

pub struct State {
    pub program: Program,
    pub data_stack: Stack<isize>,
    pub control_stack: Stack<isize>,
    pub location: (usize, usize),
    pub direction: Direction,
    pub reverse_mode: bool,
    pub string_mode: bool,
    pub multi_digit_accumulator: String,
    pub output_stack: Stack<char>,
    pub input_stack: Stack<char>,
}

impl State {
    pub fn new(program: Program) -> Self {
        let location = program.lookup();
        println!(
            "starting at location line {}, col {})",
            location.1 + 1,
            location.0 + 1
        );
        Self {
            program,
            data_stack: Stack::new(),
            control_stack: Stack::new(),
            location,
            direction: Direction::East,
            reverse_mode: false,
            string_mode: false,
            multi_digit_accumulator: String::new(),
            output_stack: Stack::new(),
            input_stack: Stack::new(),
        }
    }

    // next
    pub fn next(&self) -> (usize, usize) {
        let (x, y) = self.location;
        let (r, c) = (self.program.rows(), self.program.cols());

        // standard % operator performs the remainder operation, not a modulo
        // a % b will give a incorrect answer for negative a
        // the following function gives the correct answer
        let modulo = |a: isize, b: usize| ((a % b as isize) + b as isize) as usize % b;

        match self.direction {
            Direction::North => (x, modulo((y - 1) as isize, r)),
            Direction::East => ((x + 1) % c, y),
            Direction::South => (x, (y + 1) % r),
            Direction::West => (modulo((x - 1) as isize, c), y),
        }
    }

    // instr
    pub fn instr(&self) -> Instruction {
        let instruction = self.program
            .instruction_at(self.location)
            .expect("location in state should always give an instruction.");

        if self.reverse_mode {
            instruction.inv()
        } else {
            *instruction
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "<{P}>", P = self.program)
        write!(
            f,
            "<{D}, {C}, ({lx}, {ly}), {d}, {r}, {s}, '{n}'>",
            // P = self.program,
            D = self.data_stack,
            C = self.control_stack,
            lx = self.location.0,
            ly = self.location.1,
            d = self.direction,
            r = self.reverse_mode,
            s = self.string_mode,
            n = self.multi_digit_accumulator
        )
    }
}
