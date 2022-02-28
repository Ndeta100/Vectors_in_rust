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
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut vec = Vec::new();
        vec.push(1usize);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);
        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.len(), 5);
    }
}
