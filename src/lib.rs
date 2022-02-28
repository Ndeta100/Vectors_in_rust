use std::ptr::NonNull;
pub struct Myvec<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}
impl<T> Myvec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }
    pub fn push(&mut self) {
        if self.capacity == 0 {}
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut vec: Myvec<usize> = Myvec::new();
        // vec.push(1usize);
        // vec.push(2);
        // vec.push(3);
        // vec.push(4);
        // vec.push(5);
        assert_eq!(vec.capacity(), 0);
        assert_eq!(vec.len(), 0);
    }
}
