pub struct Stack<T: Copy> {
    s: Vec<T>,
}

impl<T: Copy> Stack<T> {
    pub fn new() -> Self {
        Stack { s: Vec::new() }
    }

    pub fn get(&self, index: usize) -> Option<T> {
        match self.s.get(index) {
            None => None,
            Some(x) => Some(*x),
        }
    }
}
