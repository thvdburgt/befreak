use std::fmt;
use std::fmt::Display;

pub struct Stack<T: Copy + Display> {
    s: Vec<T>,
}

impl<T: Copy + Display> Stack<T> {
    pub fn new() -> Self {
        Stack { s: Vec::new() }
    }

    // pub fn get(&self, index: usize) -> Option<T> {
    //     match self.s.get(index) {
    //         None => None,
    //         Some(x) => Some(*x),
    //     }
    // }

    pub fn push(&mut self, value: T) {
        self.s.push(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.s.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.s.is_empty()
    }

    pub fn last(&self) -> Option<T> {
        match self.s.last() {
            Some(&x) => Some(x),
            None => None,
        }
    }

    pub fn len(&self) -> usize {
        self.s.len()
    }

    pub fn get(&self, index: usize) -> Option<T> {
        match self.s.get(index) {
            Some(&x) => Some(x),
            None => None,
        }
    }
}

impl<T: Copy + Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.s.is_empty() {
            write!(f, "ε")
        } else {
            let mut iter = self.s.iter();
            // write first element
            let first_elem = iter.next().expect("non empty");
            match write!(f, "{}", first_elem) {
                Ok(_) => {}
                Err(e) => return Err(e),
            };
            // write the rest of the elements
            for elem in iter {
                match write!(f, ":{}", elem) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
            }

            Ok(())
        }
    }
}
