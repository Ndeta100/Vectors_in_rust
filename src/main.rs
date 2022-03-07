use rust_vectors::Myvec;

struct Dropped(usize);
impl Drop for Dropped {
    fn drop(&mut self) {
        println!("Dropping");
    }
}
fn main() {
    let mut vec: Myvec<usize> = Myvec::new();
    vec.push(1usize);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    for n in 0..vec.len() {
        assert_eq!(vec.get(3), Some(&(n + 1)));
    }

    assert_eq!(vec.capacity(), 8);
    assert_eq!(vec.len(), 5);
}
