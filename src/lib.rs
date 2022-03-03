use std::alloc;
use std::mem;
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
    pub fn push(&mut self, item: T) {
        assert_ne!(mem::size_of::<T>(), 0, "No zero sized types");
        if self.capacity == 0 {
            let layout = alloc::Layout::array::<T>(4).expect("Could not allocate memory ");
            // SAFETY: the layout is a hardcoded 4* size_of <T>
            let ptr = unsafe { alloc::alloc(layout) } as *mut T;
            let ptr = NonNull::new(ptr).expect("Coould not allocate memory ");
            //SAFETY: ptr is non-null and we have just allocated enough space for this item (and 3 more )
            //the memory previously at ptr is not read
            unsafe { ptr.as_ptr().write(item) }
            self.ptr = ptr;
            self.capacity = 4;
            self.len = 1
        } else if self.len < self.capacity {
            let offset = self
                .len
                .checked_mul(mem::size_of::<T>())
                .expect("Cannot reach memoryt location");
            //Offset cannot wrap around and pointer is pointing to valid memory
            //And writing to an offset at self.len is valid
            assert!(offset < isize::MAX as usize, "Wrapped isize");
            unsafe { self.ptr.as_ptr().add(self.len).write(item) }
            self.len += 1;
        } else {
            todo!()
        }
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
