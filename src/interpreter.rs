use instruction::InstructionExecutionStatus;
use program::Program;
use state::State;

pub fn run(program: Program) {
    println!("Running program:");
    println!("{}", program);

    let mut state = State::new(program);

    let mut counter = 0;
    loop {
        print!("{} & ", counter);
        counter += 1;
        println!("{}", state.latex_representation());
        match state.instr().execute(&mut state) {
            InstructionExecutionStatus::Successful(s) => {
                println!("    \\Rightarrow_{{[{}]}} \\\\", s);
                continue;
            }
            InstructionExecutionStatus::Unsuccessful => {
                println!(
                    "Unsuccessful execution of instruction {} at line {}, col {})",
                    state.instr(),
                    state.location.1 + 1,
                    state.location.0 + 1
                );
                println!();
                break;
            }
            InstructionExecutionStatus::Halt => {
                println!("Program halted");
                break;
            }
        }
    }
}
