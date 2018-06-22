use std::char;
use std::fmt;
use std::io::{self, Read};

use direction::Direction;
use state::State;

#[derive(Clone, Copy, PartialEq)]
pub struct Instruction {
    c: char,
}

const NOP: char = ' ';
const PUSH: char = '(';
const POP: char = ')';
const TRANSFER_TOP_DATA_CONTROL: char = '[';
const TRANSFER_TOP_CONTROL_DATA: char = ']';
const INTERCHANGE_TOPS: char = '$';
const WRITE: char = 'w';
const READ: char = 'r';
const INCREMENT: char = '\'';
const DECREMENT: char = '`';
const ADD: char = '+';
const SUBTRACT: char = '-';
const DIVIDE: char = '%';
const MULTIPLY: char = '*';
const NOT: char = '~';
const AND: char = '+';
const OR: char = '|';
const XOR: char = '#';
const ROTATE_LEFT: char = '{';
const ROTATE_RIGHT: char = '}';
const CONTROL_TOGGLE: char = '!';
const EQUAL: char = '=';
const LESS: char = 'l';
const GREATER: char = 'g';
const SWAP_TWO_TOP: char = 's';
const DIG: char = 'd';
const BURY: char = 'b';
const SWAP_FIRST_THIRD: char = 'f';
const SWAP_SECOND_THIRD: char = 'c';
const OVER: char = 'o';
const UNDER: char = 'u';
const DUPLICATE: char = ':';
const UNDUPLICATE: char = ';';
const STRING_MODE: char = '"';
const REVERSE_MODE: char = '?';
const HALT: char = '@';
const MIRROR_BACK: char = '\\';
const MIRROR_FORWARD: char = '/';
const BRANCH_EAST: char = '>';
const BRANCH_WEST: char = '<';
const BRANCH_SOUTH: char = 'v';
const BRANCH_NORTH: char = '^';

pub enum InstructionExecutionStatus {
    Successful(&'static str),
    Unsuccessful,
    Halt,
}

impl Instruction {
    pub fn from_char(c: char) -> Option<Self> {
        if c.is_ascii() && !c.is_ascii_control() {
            Some(Instruction { c })
        } else {
            None
        }
    }

    pub fn nop() -> Self {
        Instruction { c: NOP }
    }

    pub fn is_halt(self) -> bool {
        self.c == HALT
    }

    pub fn inv(self) -> Self {
        let c = match self.c {
            PUSH => POP,
            POP => PUSH,
            TRANSFER_TOP_DATA_CONTROL => TRANSFER_TOP_CONTROL_DATA,
            TRANSFER_TOP_CONTROL_DATA => TRANSFER_TOP_DATA_CONTROL,
            INCREMENT => DECREMENT,
            DECREMENT => INCREMENT,
            ADD => SUBTRACT,
            SUBTRACT => ADD,
            DIVIDE => MULTIPLY,
            MULTIPLY => DIVIDE,
            ROTATE_LEFT => ROTATE_RIGHT,
            ROTATE_RIGHT => ROTATE_LEFT,
            DIG => BURY,
            BURY => DIG,
            OVER => UNDER,
            UNDER => OVER,
            DUPLICATE => UNDUPLICATE,
            UNDUPLICATE => DUPLICATE,
            HALT => panic!(),
            i => i,
        };

        Instruction { c }
    }

    fn direction(self) -> Option<Direction> {
        match self.c {
            BRANCH_EAST => Some(Direction::East),
            BRANCH_WEST => Some(Direction::West),
            BRANCH_SOUTH => Some(Direction::South),
            BRANCH_NORTH => Some(Direction::North),
            _ => None,
        }
    }

    pub fn execute(self, state: &mut State) -> InstructionExecutionStatus {
        //println!("executing {}", self);
        use self::InstructionExecutionStatus::*;
        match self.c {
            // nop
            NOP if !state.string_mode && state.multi_digit_accumulator.is_empty() => {
                state.location = state.next();
                Successful("\\textrm{nop}")
            }
            // digit
            _ if !state.string_mode && self.c.is_digit(10) && !state.reverse_mode => {
                state.multi_digit_accumulator.push(self.c);

                state.location = state.next();
                Successful("\\textrm{digit}")
            }
            // digit_inv
            _ if !state.string_mode && self.c.is_digit(10) && state.reverse_mode => {
                state.multi_digit_accumulator.insert(0, self.c);

                state.location = state.next();
                Successful("\\textrm{digit}_{\\,\\textrm{inv}}")
            }
            // digit_end
            _ if !state.string_mode
                && !state.multi_digit_accumulator.is_empty()
                && !self.c.is_digit(10)
                && !state.data_stack.is_empty() =>
            {
                let x = state.data_stack.pop().expect("non empty");
                let n: u32 = state
                    .multi_digit_accumulator
                    .parse()
                    .expect("should only be integers");
                state.multi_digit_accumulator.clear();
                state.data_stack.push(x ^ n);

                Successful("\\textrm{digit}_{\\,\\textrm{end}}")
            }
            // push
            PUSH if !state.string_mode && state.multi_digit_accumulator.is_empty() => {
                state.data_stack.push(0);

                state.location = state.next();
                Successful("\\textrm{push}")
            }
            // pop
            POP if !state.string_mode
                && state.multi_digit_accumulator.is_empty()
                && !state.data_stack.is_empty()
                && state.data_stack.last().expect("len >= 1") == 0 =>
            {
                state.data_stack.pop().expect("non empty");

                state.location = state.next();
                Successful("\\textrm{pop}")
            }
            // transfer_1
            TRANSFER_TOP_DATA_CONTROL
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && !state.data_stack.is_empty() =>
            {
                let x = state.data_stack.pop().expect("non empty");
                state.control_stack.push(x);

                state.location = state.next();
                Successful("\\textrm{transfer_1}")
            }
            // transfer_2
            TRANSFER_TOP_CONTROL_DATA
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && !state.control_stack.is_empty() =>
            {
                let x = state.control_stack.pop().expect("non empty");
                state.data_stack.push(x);

                state.location = state.next();
                Successful("\\textrm{transfer_2}")
            }
            // interchange
            INTERCHANGE_TOPS
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && !state.data_stack.is_empty()
                    && !state.control_stack.is_empty() =>
            {
                let x = state.data_stack.pop().expect("non empty");
                let y = state.control_stack.pop().expect("non empty");
                state.data_stack.push(y);
                state.control_stack.push(x);

                state.location = state.next();
                Successful("\\textrm{interchange}")
            }
            // write
            WRITE
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && !state.reverse_mode =>
            {
                // try casting the top of the data stack to a char
                let top = state.data_stack.pop().expect("non_empty");
                match char::from_u32(top as u32) {
                    Some(c) if c.is_ascii() => {
                        state.output_stack.push(c);
                        print!("{}", c);

                        state.location = state.next();
                        Successful("\\textrm{write}")
                    }
                    _ => {
                        state.data_stack.push(top);
                        Unsuccessful
                    }
                }
            }
            // unwrite
            WRITE
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && state.reverse_mode
                    && !state.output_stack.is_empty() =>
            {
                // pop the top char of the output stack and push its ascii value on the data stack.
                let c = state.output_stack.pop().expect("non_empty");
                debug_assert!(c.is_ascii());
                state.data_stack.push(c as u32);

                Successful("\\textrm{unwrite}")
            }
            // read
            READ if !state.string_mode
                && state.multi_digit_accumulator.is_empty()
                && !state.reverse_mode =>
            {
                // FIXME: this currently sees every byte as a char.

                // if input stack is empty read a char from stdin, otherwise pop an item from the
                // input stack and use that
                let c = if state.input_stack.is_empty() {
                    // read a single byte from input
                    match io::stdin().bytes().next() {
                        Some(Ok(byte)) if byte.is_ascii() && !byte.is_ascii_control() => {
                            Some(u32::from(byte))
                        }
                        _ => None,
                    }
                } else {
                    // pop char from input stack
                    let c = state.input_stack.pop().expect("non empty");
                    debug_assert!(c.is_ascii());
                    debug_assert!(!c.is_ascii_control());

                    Some(u32::from(c))
                };

                // check if we have a character and push it to the data stack
                match c {
                    Some(c) => {
                        state.data_stack.push(c);

                        state.location = state.next();
                        Successful("\\textrm{read}")
                    }
                    None => Unsuccessful,
                }
            }
            // unread
            READ if !state.string_mode
                && state.multi_digit_accumulator.is_empty()
                && state.reverse_mode
                && !state.data_stack.is_empty() =>
            {
                let top = state.data_stack.pop().expect("non empty");
                match char::from_u32(top as u32) {
                    Some(c) if c.is_ascii() => {
                        state.input_stack.push(c);

                        state.location = state.next();
                        Successful("\\textrm{unread}")
                    }
                    _ => {
                        state.data_stack.push(top);
                        Unsuccessful
                    }
                }
            }
            // increment
            INCREMENT
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && !state.data_stack.is_empty() =>
            {
                let x = state.data_stack.pop().expect("non empty");
                state.data_stack.push(x + 1);

                state.location = state.next();
                Successful("\\textrm{increment}")
            }
            // decrement
            DECREMENT
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && !state.data_stack.is_empty() =>
            {
                let x = state.data_stack.pop().expect("non empty");
                state.data_stack.push(x - 1);

                state.location = state.next();
                Successful("\\textrm{decrement}")
            }
            // add
            ADD if !state.string_mode
                && state.multi_digit_accumulator.is_empty()
                && state.data_stack.len() >= 2 =>
            {
                let x = state.data_stack.pop().expect("len >= 2");
                let y = state.data_stack.pop().expect("len >= 2");
                state.data_stack.push(y + x);
                state.data_stack.push(x);

                state.location = state.next();
                Successful("\\textrm{add}")
            }
            // subtract
            SUBTRACT
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && state.data_stack.len() >= 2 =>
            {
                let x = state.data_stack.pop().expect("len >= 2");
                let y = state.data_stack.pop().expect("len >= 2");
                state.data_stack.push(y - x);
                state.data_stack.push(x);

                state.location = state.next();
                Successful("\\textrm{subtract}")
            }
            // divide
            DIVIDE
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && state.data_stack.len() >= 2
                    && state.data_stack.last().expect("len >= 2") != 0 =>
            {
                let x = state.data_stack.pop().expect("len >= 2");
                let y = state.data_stack.pop().expect("len >= 2");
                state.data_stack.push(y / x);
                state.data_stack.push(y % x);
                state.data_stack.push(x);

                state.location = state.next();
                Successful("\\textrm{divide}")
            }
            // multiply
            MULTIPLY
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && state.data_stack.len() >= 3 =>
            {
                let x = state.data_stack.pop().expect("len >= 3");
                let y = state.data_stack.pop().expect("len >= 3");
                let z = state.data_stack.pop().expect("len >= 3");
                state.data_stack.push((z * x) + y);
                state.data_stack.push(x);

                state.location = state.next();
                Successful("\\textrm{multiply}")
            }
            // not
            NOT if !state.string_mode
                && state.multi_digit_accumulator.is_empty()
                && !state.data_stack.is_empty() =>
            {
                let x = state.data_stack.pop().expect("non empty");
                state.data_stack.push(!x);

                state.location = state.next();
                Successful("\\textrm{not}")
            }
            // and
            AND if !state.string_mode
                && state.multi_digit_accumulator.is_empty()
                && state.data_stack.len() >= 3 =>
            {
                let x = state.data_stack.pop().expect("len >= 3");
                let y = state.data_stack.pop().expect("len >= 3");
                let z = state.data_stack.pop().expect("len >= 3");
                state.data_stack.push((x & y) ^ z);
                state.data_stack.push(y);
                state.data_stack.push(x);

                state.location = state.next();
                Successful("\\textrm{and}")
            }
            // or
            OR if !state.string_mode
                && state.multi_digit_accumulator.is_empty()
                && state.data_stack.len() >= 3 =>
            {
                let x = state.data_stack.pop().expect("len >= 3");
                let y = state.data_stack.pop().expect("len >= 3");
                let z = state.data_stack.pop().expect("len >= 3");
                state.data_stack.push((x | y) ^ z);
                state.data_stack.push(y);
                state.data_stack.push(x);

                state.location = state.next();
                Successful("\\textrm{or}")
            }
            // xor
            XOR if !state.string_mode
                && state.multi_digit_accumulator.is_empty()
                && state.data_stack.len() >= 2 =>
            {
                let x = state.data_stack.pop().expect("len >= 2");
                let y = state.data_stack.pop().expect("len >= 2");
                state.data_stack.push(x ^ y);
                state.data_stack.push(x);

                state.location = state.next();
                Successful("\\textrm{xor}")
            }
            // rotate_left
            ROTATE_LEFT
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && state.data_stack.len() >= 2 =>
            {
                let x = state.data_stack.pop().expect("len >= 2");
                let y = state.data_stack.pop().expect("len >= 2");
                state.data_stack.push(y.rotate_left(x));
                state.data_stack.push(x);

                state.location = state.next();
                Successful("\\textrm{rotate}_{\\,\\textrm{left}}")
            }
            // rotate_right
            ROTATE_RIGHT
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && state.data_stack.len() >= 2 =>
            {
                let x = state.data_stack.pop().expect("len >= 2");
                let y = state.data_stack.pop().expect("len >= 2");
                state.data_stack.push(y.rotate_right(x));
                state.data_stack.push(x);

                state.location = state.next();
                Successful("\\textrm{}")
            }
            // toggle
            CONTROL_TOGGLE
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && !state.control_stack.is_empty() =>
            {
                let c = state.control_stack.pop().expect("non empty");
                state.control_stack.push(c ^ 1);

                state.location = state.next();
                Successful("\\textrm{toggle}")
            }
            // equal_true / equal_false
            EQUAL
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && state.data_stack.len() >= 2
                    && !state.control_stack.is_empty() =>
            {
                let x = state
                    .data_stack
                    .get(state.data_stack.len() - 1)
                    .expect("len >= 2");
                let y = state
                    .data_stack
                    .get(state.data_stack.len() - 2)
                    .expect("len >= 2");
                if y == x {
                    let c = state.control_stack.pop().expect("non empty");
                    state.control_stack.push(c ^ 1);
                    state.location = state.next();
                    Successful("\\textrm{equal}_{\\,\\textrm{true}}")
                } else {
                    state.location = state.next();
                    Successful("\\textrm{equal}_{\\,\\textrm{false}}")
                }
            }
            // less_true / less_false
            LESS if !state.string_mode
                && state.multi_digit_accumulator.is_empty()
                && state.data_stack.len() >= 2
                && !state.control_stack.is_empty() =>
            {
                let x = state
                    .data_stack
                    .get(state.data_stack.len() - 1)
                    .expect("len >= 2");
                let y = state
                    .data_stack
                    .get(state.data_stack.len() - 2)
                    .expect("len >= 2");
                if y < x {
                    let c = state.control_stack.pop().expect("non empty");
                    state.control_stack.push(c ^ 1);
                    state.location = state.next();
                    Successful("\\textrm{equal}_{\\,\\textrm{true}}")
                } else {
                    state.location = state.next();
                    Successful("\\textrm{equal}_{\\,\\textrm{false}}")
                }
            }
            // greater_true / greater_false
            GREATER
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && state.data_stack.len() >= 2
                    && !state.control_stack.is_empty() =>
            {
                let x = state
                    .data_stack
                    .get(state.data_stack.len() - 1)
                    .expect("len >= 2");
                let y = state
                    .data_stack
                    .get(state.data_stack.len() - 2)
                    .expect("len >= 2");
                if y > x {
                    let c = state.control_stack.pop().expect("non empty");
                    state.control_stack.push(c ^ 1);
                    state.location = state.next();
                    Successful("\\textrm{greater}_{\\,\\textrm{true}}")
                } else {
                    state.location = state.next();
                    Successful("\\textrm{greater}_{\\,\\textrm{false}}")
                }
            }
            // swap_1
            SWAP_TWO_TOP
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && state.data_stack.len() >= 2 =>
            {
                let x = state.data_stack.pop().expect("len >= 2");
                let y = state.data_stack.pop().expect("len >= 2");
                state.data_stack.push(x);
                state.data_stack.push(y);

                state.location = state.next();
                Successful("\\textrm{swap}_{\\,\\textrm{1}}")
            }
            // dig
            DIG if !state.string_mode
                && state.multi_digit_accumulator.is_empty()
                && state.data_stack.len() >= 3 =>
            {
                let x = state.data_stack.pop().expect("len >= 3");
                let y = state.data_stack.pop().expect("len >= 3");
                let z = state.data_stack.pop().expect("len >= 3");
                state.data_stack.push(y);
                state.data_stack.push(x);
                state.data_stack.push(z);

                state.location = state.next();
                Successful("\\textrm{dig}")
            }
            // bury
            BURY if !state.string_mode
                && state.multi_digit_accumulator.is_empty()
                && state.data_stack.len() >= 3 =>
            {
                let x = state.data_stack.pop().expect("len >= 3");
                let y = state.data_stack.pop().expect("len >= 3");
                let z = state.data_stack.pop().expect("len >= 3");
                state.data_stack.push(x);
                state.data_stack.push(z);
                state.data_stack.push(y);

                state.location = state.next();
                Successful("\\textrm{bury}")
            }
            // swap_3
            SWAP_FIRST_THIRD
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && state.data_stack.len() >= 3 =>
            {
                let x = state.data_stack.pop().expect("len >= 3");
                let y = state.data_stack.pop().expect("len >= 3");
                let z = state.data_stack.pop().expect("len >= 3");
                state.data_stack.push(x);
                state.data_stack.push(y);
                state.data_stack.push(z);

                state.location = state.next();
                Successful("\\textrm{swap}_{\\,\\textrm{3}}")
            }
            // swap_2
            SWAP_SECOND_THIRD
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && state.data_stack.len() >= 3 =>
            {
                let x = state.data_stack.pop().expect("len >= 3");
                let y = state.data_stack.pop().expect("len >= 3");
                let z = state.data_stack.pop().expect("len >= 3");
                state.data_stack.push(y);
                state.data_stack.push(z);
                state.data_stack.push(x);

                state.location = state.next();
                Successful("\\textrm{swap}_{\\,\\textrm{2}}")
            }
            // over
            OVER if !state.string_mode
                && state.multi_digit_accumulator.is_empty()
                && state.data_stack.len() >= 2 =>
            {
                let x = state.data_stack.pop().expect("len >= 2");
                let y = state.data_stack.pop().expect("len >= 2");
                state.data_stack.push(y);
                state.data_stack.push(x);
                state.data_stack.push(y);

                state.location = state.next();
                Successful("\\textrm{over}")
            }
            // under
            UNDER
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && state.data_stack.len() >= 3
                    && state.data_stack.last().expect("len >= 3")
                        == state
                            .data_stack
                            .get(state.data_stack.len() - 3)
                            .expect("len >= 3") =>
            {
                let y = state.data_stack.pop().expect("len >= 3");
                let x = state.data_stack.pop().expect("len >= 3");
                debug_assert!(state.data_stack.last().expect("len >= 3") == y);
                state.data_stack.push(x);

                state.location = state.next();
                Successful("\\textrm{under}")
            }
            // duplicate
            DUPLICATE
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && !state.data_stack.is_empty() =>
            {
                let x = state.data_stack.pop().expect("non empty");
                state.data_stack.push(x);
                state.data_stack.push(x);

                state.location = state.next();
                Successful("\\textrm{duplicate}")
            }
            // unduplicate
            UNDUPLICATE
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && state.data_stack.len() >= 2
                    && state.data_stack.last().expect("len >= 2")
                        == state
                            .data_stack
                            .get(state.data_stack.len() - 2)
                            .expect("len >= 2") =>
            {
                state.data_stack.pop().expect("len >= 2");

                state.location = state.next();
                Successful("\\textrm{unduplicate}")
            }
            // string_toggle
            STRING_MODE if state.multi_digit_accumulator.is_empty() => {
                state.string_mode = !state.string_mode;

                state.location = state.next();
                Successful("\\textrm{string}_{\\,\\textrm{toggle}}")
            }
            // halt
            HALT if !state.string_mode && state.multi_digit_accumulator.is_empty() => Halt,
            // mirror_1
            MIRROR_BACK if !state.string_mode && state.multi_digit_accumulator.is_empty() => {
                state.direction = state.direction.mirror();

                state.location = state.next();
                Successful("\\textrm{mirror}_{\\,\\textrm{1}}")
            }
            // mirror_2
            MIRROR_FORWARD if !state.string_mode && state.multi_digit_accumulator.is_empty() => {
                state.direction = state.direction.mirror().opposite();

                state.location = state.next();
                Successful("\\textrm{mirror}_{\\,\\textrm{2}}")
            }
            // branch_1
            BRANCH_EAST | BRANCH_NORTH | BRANCH_SOUTH | BRANCH_WEST
                if !state.string_mode && state.multi_digit_accumulator.is_empty()
                    && (self.direction().expect("self is a branching instruction")
                        == state.direction.right()
                        || self.direction().expect("self is a branching instruction")
                            == state.direction.left()) =>
            {
                // IP coming from the side
                let d = self.direction().expect("self is a branching instruction");
                if d == state.direction.right() {
                    // right turn
                    state
                        .control_stack
                        .push(if !state.reverse_mode { 1 } else { 0 });
                } else if d == state.direction.left() {
                    // left turn
                    state
                        .control_stack
                        .push(if !state.reverse_mode { 0 } else { 1 });
                } else {
                    panic!("incoming direction should be side of instuction");
                }
                state.direction = d;

                state.location = state.next();
                Successful("\\textrm{branch}_{\\,\\textrm{1}}")
            }
            // branch_2
            BRANCH_EAST | BRANCH_NORTH | BRANCH_SOUTH | BRANCH_WEST
                if !state.string_mode && state.multi_digit_accumulator.is_empty()
                    && state.direction
                        == self.direction()
                            .expect("self is a branching instruction")
                            .opposite() && !state.control_stack.is_empty() =>
            {
                // IP coming from the opposite direction
                let c = state.control_stack.pop().expect("non empty");
                // Turn left if c equals 0 (opposite in reverse mode), and turn right if 1 (opposite in reverse mode)
                if (c == 0) ^ state.reverse_mode {
                    state.direction = state.direction.left();
                } else {
                    state.direction = state.direction.right();
                }

                state.location = state.next();
                Successful("\\textrm{branch}_{\\,\\textrm{2}}")
            }
            // branch_3
            BRANCH_EAST | BRANCH_NORTH | BRANCH_SOUTH | BRANCH_WEST
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && state.direction == self.direction().expect("self is a branching instruction")
                    && !state.control_stack.is_empty() =>
            {
                // IP coming from the same direction
                let c = state.control_stack.pop().expect("non empty");
                // toggle top of the control stack
                state.control_stack.push(c ^ 1);
                // toggle reverse mode
                state.reverse_mode = !state.reverse_mode;
                // go in the opposite direction
                state.direction = state.direction.opposite();

                state.location = state.next();
                Successful("\\textrm{branch}_{\\,\\textrm{3}}")
            }
            // string_push
            _ if state.string_mode
                && state.multi_digit_accumulator.is_empty()
                && !state.reverse_mode =>
            {
                // if reverse mode is not enabled push the ascii value of the char to the
                // stack
                state.data_stack.push(self.c as u32);

                state.location = state.next();
                Successful("\\textrm{string}_{\\,\\textrm{push}}")
            }
            // string_pop
            _ if state.string_mode
                && state.multi_digit_accumulator.is_empty()
                && state.reverse_mode
                && !state.data_stack.is_empty()
                && state.data_stack.last().expect("non empty") == self.c as u32 =>
            {
                state.data_stack.pop();

                state.location = state.next();
                Successful("\\textrm{string}_{\\,\\textrm{pop}}")
            }
            // reverse
            REVERSE_MODE if !state.string_mode && state.multi_digit_accumulator.is_empty() => {
                state.reverse_mode = !state.reverse_mode;

                state.location = state.next();
                Successful("\\textrm{reverse}")
            }
            // no rule found
            _ => {
                println!("No rule found");
                Unsuccessful
            }
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.c.fmt(f)
    }
}
