use direction::Direction;
use instruction::Instruction;
use program::Program;
use stack::Stack;

pub struct State {
    pub program: Program,
    pub data_stack: Vec<isize>,
    pub control_stack: Vec<isize>,
    pub location: (usize, usize),
    pub direction: Direction,
    pub reverse_mode: bool,
    pub string_mode: bool,
    pub multi_digit_accumulator: Vec<char>,
}

impl State {
    pub fn new(program: Program) -> Self {
        let location = program.lookup();
        Self {
            program,
            data_stack: Vec::new(),
            control_stack: Vec::new(),
            location,
            direction: Direction::East,
            reverse_mode: false,
            string_mode: false,
            multi_digit_accumulator: Vec::new(),
        }
    }

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

    pub fn instr(&self) -> Instruction {
        let instruction = self
            .program
            .instruction_at(self.location)
            .expect("location in state should always give an instruction.");

        if self.reverse_mode {
            *instruction
        } else {
            instruction.inv()
        }
    }
}
