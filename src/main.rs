use rust_vectors::Myvec;
fn main() {
    let mut vec: Myvec<usize> = Myvec::new();
    vec.push(1usize);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    assert_eq!(vec.capacity(), 8);
    assert_eq!(vec.len(), 5);
}
