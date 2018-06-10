use program::Program;
use state::State;

pub fn run(program: Program) {
    println!("Running program:");
    println!("{}", program);

    // TODO: return if execution was successful
    let mut state = State::new(program);

    loop {
        if !step(&mut state) {
            break;
        }
    }
}

fn step(state: &mut State) -> bool {
    // let instr = state.instr();

    // // deconstruct the state into shorted variable names
    // let State {
    //     program: _,
    //     data_stack: ds,
    //     control_stack: cs,
    //     location: l,
    //     direction: d,
    //     reverse_mode: r,
    //     string_mode: s,
    //     multi_digit_accumulator: n,
    // } = state;

    // {
    //     use instruction::Instruction::*;
    //     match (instr, ds, cs, l, d, r, s, n) {
    //         // nop
    //         (Nop, ds, cs, l, d, r, s, n) => true,
    //         // TODO: digit
    //         // TODO: digit inv
    //         // addition
    //         (Addition, ds, cs, l, d, r, s, n) => {
    //             if ds.len() < 2 {
    //                 false
    //             } else {
    //                 let x = ds.pop();
    //                 let y = ds.pop();

    //                 true
    //             }
    //         },
    //         _ => false,
    //     }
    // }
    false
}
