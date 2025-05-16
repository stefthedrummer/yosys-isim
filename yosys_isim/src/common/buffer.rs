use std::ops::Deref;
use std::ops::DerefMut;

pub struct EmptyBuffer<T> {
    buffer: Vec<T>,
}

impl<T> EmptyBuffer<T> {
    pub fn new() -> Self {
        EmptyBuffer { buffer: Vec::new() }
    }
}

impl<T: Clone> EmptyBuffer<T> {
    pub fn prepare(mut self, len: usize, init: T) -> Buffer<T> {
        self.buffer.resize(len, init);

        Buffer {
            buffer: self.buffer,
        }
    }
}

pub struct Buffer<T> {
    buffer: Vec<T>,
}



impl<T> Deref for Buffer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl<T> DerefMut for Buffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}
