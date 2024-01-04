use std::mem::swap;

pub struct CircularBuffer<T>
where
    T: Default + Clone,
{
    data: Vec<T>,
    n: usize,
    read_index: usize,
    write_index: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T>
where
    T: Default + Clone,
{
    pub fn new(capacity: usize) -> Self {
        let capacity_plus_one = capacity + 1;
        Self {
            data: vec![T::default(); capacity_plus_one],
            n: capacity_plus_one,
            write_index: 0,
            read_index: 0,
        }
    }

    fn increment_write_index(&mut self) {
        self.write_index = (self.write_index + 1) % self.n;
    }

    fn increment_read_index(&mut self) {
        self.read_index = (self.read_index + 1) % self.n;
    }

    fn is_empty(&self) -> bool {
        self.read_index == self.write_index
    }

    fn is_full(&self) -> bool {
        (self.write_index + 1) % self.n == self.read_index
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer);
        }

        self.data[self.write_index] = _element;
        self.increment_write_index();

        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            return Err(Error::EmptyBuffer);
        }

        let mut elem: T = T::default();
        swap(&mut self.data[self.read_index], &mut elem);
        self.increment_read_index();

        Ok(elem)
    }

    pub fn clear(&mut self) {
        self.data = vec![T::default(); self.n];
        self.read_index = 0;
        self.write_index = 0;
    }

    pub fn overwrite(&mut self, _element: T) {
        self.data[self.write_index] = _element;
        if self.is_full() {
            self.increment_read_index();
        }
        self.increment_write_index();
    }
}
