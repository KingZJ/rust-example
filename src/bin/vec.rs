fn main() {
    let arr = vec![1, 2, 3];
    println!("{:#?} {0:?} {:?}", arr, arr.capacity());

    let mut arr: Vec<u32> = Vec::with_capacity(10);
    arr.push(1);
    arr.push(2);
    arr.push(3);
    arr.push(4);
    println!("{:?} {:?} {:?}", arr, arr.len(), arr.capacity());
}
