use std::fmt;

use direction::Direction;
use state::State;


// type Instruction = char;

#[derive(Clone, Copy, PartialEq)]
pub enum Instruction {
    Nop,
    Digit(u32),
    Push,
    Pop,
    TransferTopDataControl,
    TransferTopControlData,
    InterchangeTops,
    // Write,      // unimplemented
    // Read,       // unimplemented
    Increment,
    Decrement,
    Add,
    Subtract,
    Divide,
    Multiply,
    Not,
    And,
    Or,
    Xor,
    RotateLeft,
    RotateRight,
    ControlToggle,
    Equals,
    Less,
    Greater,
    SwapTopTwo,
    Dig,
    Bury,
    Flip,
    SwapSecondThird,
    Over,
    Under,
    Duplicate,
    Unduplicate,
    StringMode,
    ReverseMode,
    Halt,
    MirrorBack,    // Like backslash \
    MirrorForward, // Like forward slash /
    Branch(Direction),
    AsciiPrintable(char),
}

impl Instruction {
    // TODO: find a better solution for the double associating in from_char/to_char.
    pub fn from_char(c: char) -> Option<Self> {
        use self::Instruction::*;
        match c {
            ' ' => Some(Nop),
            '0'..='9' => Some(Digit(c.to_digit(10).expect("c must be a character digit"))),
            '(' => Some(Push),
            ')' => Some(Pop),
            '[' => Some(TransferTopDataControl),
            ']' => Some(TransferTopControlData),
            '$' => Some(InterchangeTops),
            //'w' => Some(Write),
            //'r' => Some(Read),
            '\'' => Some(Increment),
            '`' => Some(Decrement),
            '+' => Some(Add),
            '-' => Some(Subtract),
            '%' => Some(Divide),
            '*' => Some(Multiply),
            '~' => Some(Not),
            '&' => Some(And),
            '|' => Some(Or),
            '#' => Some(Xor),
            '{' => Some(RotateLeft),
            '}' => Some(RotateRight),
            '!' => Some(ControlToggle),
            '=' => Some(Equals),
            'l' => Some(Less),
            'g' => Some(Greater),
            's' => Some(SwapTopTwo),
            'd' => Some(Dig),
            'b' => Some(Bury),
            'f' => Some(Flip),
            'c' => Some(SwapSecondThird),
            'o' => Some(Over),
            'u' => Some(Under),
            ':' => Some(Duplicate),
            ';' => Some(Unduplicate),
            '"' => Some(StringMode),
            '?' => Some(ReverseMode),
            '@' => Some(Halt),
            '\\' => Some(MirrorBack),
            '/' => Some(MirrorForward),
            '>' => Some(Branch(Direction::East)),
            '<' => Some(Branch(Direction::West)),
            'v' => Some(Branch(Direction::South)),
            '^' => Some(Branch(Direction::North)),
            c if c.is_ascii() && !c.is_ascii_control() => Some(AsciiPrintable(c)),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        use self::Instruction::*;
        use std::char;
        match self {
            Nop => ' ',
            Digit(x) => char::from_digit(*x, 10).expect("x is not a digit"),
            Push => '(',
            Pop => ')',
            TransferTopDataControl => '[',
            TransferTopControlData => ']',
            InterchangeTops => '$',
            // Write => 'w',
            // Read => 'r',
            Increment => '\'',
            Decrement => '`',
            Add => '+',
            Subtract => '-',
            Divide => '%',
            Multiply => '*',
            Not => '~',
            And => '+',
            Or => '|',
            Xor => '#',
            RotateLeft => '{',
            RotateRight => '}',
            ControlToggle => '!',
            Equals => '=',
            Less => 'l',
            Greater => 'g',
            SwapTopTwo => 's',
            Dig => 'd',
            Bury => 'b',
            Flip => 'f',
            SwapSecondThird => 'c',
            Over => 'o',
            Under => 'u',
            Duplicate => ':',
            Unduplicate => ';',
            StringMode => '"',
            ReverseMode => '?',
            Halt => '@',
            MirrorBack => '\\',
            MirrorForward => '/',
            Branch(Direction::East) => '>',
            Branch(Direction::West) => '<',
            Branch(Direction::South) => 'v',
            Branch(Direction::North) => '^',
            AsciiPrintable(c) => *c,
        }
    }

    pub fn inv(&self) -> Self {
        use self::Instruction::*;
        match *self {
            Push => Pop,
            Pop => Push,
            TransferTopDataControl => TransferTopControlData,
            TransferTopControlData => TransferTopDataControl,
            Increment => Decrement,
            Decrement => Increment,
            Add => Subtract,
            Subtract => Add,
            Divide => Multiply,
            Multiply => Divide,
            RotateLeft => RotateRight,
            RotateRight => RotateLeft,
            Dig => Bury,
            Bury => Dig,
            Over => Under,
            Under => Over,
            Duplicate => Unduplicate,
            Unduplicate => Duplicate,
            Halt => panic!(),
            i => i,
        }
    }

    pub fn execute(&self, state: &mut State) -> bool {
        use self::Instruction::*;
        match self {
            Nop if !state.string_mode && state.multi_digit_accumulator.is_empty() => {
                state.location = state.next();
                true
            }
            // Digit(x) => char::from_digit(*x, 10).expect("x is not a digit"),
            Push => {
                state.data_stack.push(0);

                state.location = state.next();
                true
            }
            Pop if !state.string_mode
                && state.multi_digit_accumulator.is_empty()
                && !state.data_stack.is_empty()
                && *state.data_stack.last().expect("len >= 1") == 0 =>
            {
                state.data_stack.pop().expect("non empty");

                state.location = state.next();
                true
            }
            TransferTopDataControl
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && !state.data_stack.is_empty() =>
            {
                let x = state.data_stack.pop().expect("non empty");
                state.control_stack.push(x);

                state.location = state.next();
                true
            }
            TransferTopControlData
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && !state.control_stack.is_empty() =>
            {
                let x = state.control_stack.pop().expect("non empty");
                state.data_stack.push(x);

                state.location = state.next();
                true
            }
            InterchangeTops
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
                true
            }
            // // Write => 'w',
            // // Read => 'r',
            Increment
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && !state.data_stack.is_empty() =>
            {
                let x = state.data_stack.pop().expect("non empty");
                state.data_stack.push(x + 1);

                state.location = state.next();
                true
            }
            Decrement
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && !state.data_stack.is_empty() =>
            {
                let x = state.data_stack.pop().expect("non empty");
                state.data_stack.push(x - 1);

                state.location = state.next();
                true
            }
            Add if !state.string_mode
                && state.multi_digit_accumulator.is_empty()
                && state.data_stack.len() >= 2 =>
            {
                let x = state.data_stack.pop().expect("len >= 2");
                let y = state.data_stack.pop().expect("len >= 2");
                state.data_stack.push(y + x);
                state.data_stack.push(x);

                state.location = state.next();
                true
            }
            Subtract
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && state.data_stack.len() >= 2 =>
            {
                let x = state.data_stack.pop().expect("len >= 2");
                let y = state.data_stack.pop().expect("len >= 2");
                state.data_stack.push(y - x);
                state.data_stack.push(x);

                state.location = state.next();
                true
            }
            Divide
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && state.data_stack.len() >= 2
                    && *state.data_stack.last().expect("len >= 2") != 0 =>
            {
                let x = state.data_stack.pop().expect("len >= 2");
                let y = state.data_stack.pop().expect("len >= 2");
                state.data_stack.push(y / x);
                state.data_stack.push(y % x);
                state.data_stack.push(x);

                state.location = state.next();
                true
            }
            Multiply
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
                true
            }
            Not if !state.string_mode
                && state.multi_digit_accumulator.is_empty()
                && !state.data_stack.is_empty() =>
            {
                let x = state.data_stack.pop().expect("non empty");
                state.data_stack.push(!x);

                state.location = state.next();
                true
            }
            And if !state.string_mode
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
                true
            }
            Or if !state.string_mode
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
                true
            }
            Xor if !state.string_mode
                && state.multi_digit_accumulator.is_empty()
                && state.data_stack.len() >= 2 =>
            {
                let x = state.data_stack.pop().expect("len >= 2");
                let y = state.data_stack.pop().expect("len >= 2");
                state.data_stack.push(x ^ y);
                state.data_stack.push(x);

                state.location = state.next();
                true
            }
            RotateLeft
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && state.data_stack.len() >= 2 =>
            {
                let x = state.data_stack.pop().expect("len >= 2");
                let y = state.data_stack.pop().expect("len >= 2");
                state.data_stack.push(y.rotate_left(x as u32)); // TODO: range check
                state.data_stack.push(x);

                state.location = state.next();
                true
            }
            RotateRight
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && state.data_stack.len() >= 2 =>
            {
                let x = state.data_stack.pop().expect("len >= 2");
                let y = state.data_stack.pop().expect("len >= 2");
                state.data_stack.push(y.rotate_right(x as u32)); // TODO: range check
                state.data_stack.push(x);

                state.location = state.next();
                true
            }
            Control
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && !state.control_stack.is_empty() =>
            {
                let c = state.control_stack.pop().expect("non empty");
                state.control_stack.push(c ^ 1);

                state.location = state.next();
                true
            }
            Equals
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && state.data_stack.len() >= 2
                    && !state.control_stack.is_empty() =>
            {
                let x = state.data_stack.pop().expect("len >= 2");
                let y = state.data_stack.pop().expect("len >= 2");
                if y == x {
                    let c = state.control_stack.pop().expect("non empty");
                    state.control_stack.push(c ^ 1);
                }

                state.location = state.next();
                true
            }
            Less if !state.string_mode
                && state.multi_digit_accumulator.is_empty()
                && state.data_stack.len() >= 2
                && !state.control_stack.is_empty() =>
            {
                let x = state.data_stack.pop().expect("len >= 2");
                let y = state.data_stack.pop().expect("len >= 2");
                if y < x {
                    let c = state.control_stack.pop().expect("non empty");
                    state.control_stack.push(c ^ 1);
                }

                state.location = state.next();
                true
            }
            Greater
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && state.data_stack.len() >= 2
                    && !state.control_stack.is_empty() =>
            {
                let x = state.data_stack.pop().expect("len >= 2");
                let y = state.data_stack.pop().expect("len >= 2");
                if y > x {
                    let c = state.control_stack.pop().expect("non empty");
                    state.control_stack.push(c ^ 1);
                }

                state.location = state.next();
                true
            }
            SwapTopTwo
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && state.data_stack.len() >= 2 =>
            {
                let x = state.data_stack.pop().expect("len >= 2");
                let y = state.data_stack.pop().expect("len >= 2");
                state.data_stack.push(x);
                state.data_stack.push(y);

                state.location = state.next();
                true
            }
            Dig if !state.string_mode
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
                true
            }
            Bury if !state.string_mode
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
                true
            }
            Flip if !state.string_mode
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
                true
            }
            SwapSecondThird
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
                true
            }
            Over if !state.string_mode
                && state.multi_digit_accumulator.is_empty()
                && state.data_stack.len() >= 2 =>
            {
                let x = state.data_stack.pop().expect("len >= 2");
                let y = state.data_stack.pop().expect("len >= 2");
                state.data_stack.push(y);
                state.data_stack.push(x);
                state.data_stack.push(y);

                state.location = state.next();
                true
            }
            Under
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && state.data_stack.len() >= 3
                    && *state.data_stack.last().expect("len >= 3")
                        == state.data_stack[state.data_stack.len() - 3] =>
            {
                let y = state.data_stack.pop().expect("len >= 3");
                let x = state.data_stack.pop().expect("len >= 3");
                debug_assert!(*state.data_stack.last().expect("len >= 3") == y);
                state.data_stack.push(x);

                state.location = state.next();
                true
            }
            Duplicate
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && !state.data_stack.is_empty() =>
            {
                let x = state.data_stack.pop().expect("non empty");
                state.data_stack.push(x);
                state.data_stack.push(x);

                state.location = state.next();
                true
            }
            Unduplicate
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty()
                    && state.data_stack.len() >= 2
                    && *state.data_stack.last().expect("len >= 2")
                        == state.data_stack[state.data_stack.len() - 2] =>
            {
                state.data_stack.pop().expect("len >= 2");

                state.location = state.next();
                true
            }
            StringMode
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty() =>
            {
                state.string_mode = !state.string_mode;

                state.location = state.next();
                true
            }
            ReverseMode
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty() =>
            {
                state.reverse_mode = !state.reverse_mode;

                state.location = state.next();
                true
            }
            // TODO: Halt => '@',
            MirrorBack
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty() =>
            {
                state.direction = state.direction.mirror();

                state.location = state.next();
                true
            }
            MirrorForward
                if !state.string_mode
                    && state.multi_digit_accumulator.is_empty() =>
            {
                state.direction = state.direction.mirror().opposite();

                state.location = state.next();
                true
            }
            Branch(d) => {
                if state.direction == *d {
                    // IP coming from the same direction
                    if state.control_stack.len() < 1 {
                        false
                    } else {
                        let c = state.control_stack.pop().expect("len >= 1");
                        // toggle top of the control stack
                        state.control_stack.push(c ^ 1);
                        // toggle reverse mode
                        state.reverse_mode = !state.reverse_mode;
                        // go in the opposite direction
                        state.direction = state.direction.opposite();

                        state.location = state.next();
                        true
                    }
                } else if state.direction == d.opposite() {
                    // IP coming from the opposite direction
                    if state.control_stack.len() < 1 {
                        false
                    } else {
                        let c = state.control_stack.pop().expect("len >= 1");
                        // Turn left if c equals 0 (opposite in reverse mode), and turn
                        // right if 1 (opposite in reverse mode)
                        if (c == 0) ^ state.reverse_mode {
                            state.direction = state.direction.left();
                        } else {
                            state.direction = state.direction.right();
                        }

                        state.location = state.next();
                        true
                    }
                } else {
                    // IP coming from the side
                    if *d == state.direction.right() {
                        // right turn
                        state
                            .control_stack
                            .push(if !state.reverse_mode { 0 } else { 1 });
                    } else if *d == state.direction.left() {
                        // left turn
                        state
                            .control_stack
                            .push(if !state.reverse_mode { 1 } else { 0 });
                    } else {
                        panic!("incomming direction should be side of instuction");
                    }

                    state.direction = *d;
                    false
                }
            }
            AsciiPrintable(c) => {
                if state.string_mode {
                    let c = *c as isize;
                    if !state.reverse_mode {
                        // if reverse mode is not enabled push the ascii value of the char to the
                        // stack
                        state.data_stack.push(c);
                        state.location = state.next();
                        true
                    } else {
                        // if reverse mode is enabled pop the ascii value of the char from the
                        // stack
                        if state.data_stack.len() < 1
                            || *state.data_stack.last().expect("len >= 1") != c
                        {
                            false
                        } else {
                            state.data_stack.pop();
                            true
                        }
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
