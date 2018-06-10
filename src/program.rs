use std::fmt;

use instruction::Instruction;

pub struct Program {
    instructions: Vec<Vec<Instruction>>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        // TODO: make Result
        // try converting the string to a 2d-vec of instructions
        // if this fails return None

        let mut instructions = {
            let mut rows = Vec::new();
            for (x, line) in s.lines().enumerate() {
                let mut row = Vec::new();
                for (y, c) in line.chars().enumerate() {
                    let i = match Instruction::from_char(c) {
                        Some(i) => i,
                        None => {
                            eprintln!("unable to parse \'{}\' at {}:{}", c, x, y);
                            return None;
                        }
                    };
                    row.push(i);
                }
                rows.push(row);
            }

            rows
        };

        // determine width of the program
        let width = {
            use std::cmp;
            instructions
                .iter()
                .fold(0, |width, row| cmp::max(width, row.len()))
        };

        // pad rows that are not of length width
        for row in instructions.iter_mut() {
            for _ in 0..(width - row.len()) {
                row.push(Instruction::Nop);
            }
        }

        // assert that all rows are of equal length
        debug_assert!(instructions.iter().all(|ref row| row.len() == width));

        Some(Program { instructions })
    }

    pub fn rows(&self) -> usize {
        self.instructions.len()
    }

    pub fn cols(&self) -> usize {
        self.instructions.first().map_or(0, |v| v.len())
    }

    pub fn instruction_at(&self, location: (usize, usize)) -> Option<&Instruction> {
        let (x, y) = location;
        self.instructions.get(y)?.get(x)
    }

    pub fn lookup(&self) -> (usize, usize) {
        // Loop through the program, bottom to top, left to right, looking for @ (the start symbol)
        let mut last_halt = None;
        for (y, row) in self.instructions.iter().enumerate() {
            for (x, ins) in row.iter().enumerate() {
                if *ins == Instruction::Halt {
                    last_halt = Some((x, y));
                }
            }
        }

        // If no start symbol occurs start at the top left
        // otherwise start one symbol to the east of the start symbol
        last_halt.map_or((0, 0), |(x, y)| ((x + 1) % self.cols(), y))
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.instructions.iter() {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
