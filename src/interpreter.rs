use instruction::InstructionExecutionStatus;
use program::Program;
use state::State;

pub fn run(program: Program) {
    println!("Running program:");
    println!("{}", program);

    let mut state = State::new(program);

    loop {
        //println!("{}", state);
        match state.instr().execute(&mut state) {
            InstructionExecutionStatus::Successful => continue,
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
