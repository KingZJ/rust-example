pub fn test() {
    println!("\n........slice type exercise......");

    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];
    println!("{:?} {:?}", arr, s1);
}