pub struct Stack<T> {
    pub vec: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Self { vec: vec![] }
    }

    pub fn push(&mut self, val: T) {
        self.vec.push(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }

    pub fn top(&mut self) -> Option<&T> {
        self.vec.last()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}
