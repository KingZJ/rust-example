pub fn test() {
    println!("\n........array type exercise......");

    let arr0: [i32; 5] = [1; 5];  // error: [String::from("a"); 5]
    println!("{arr0:?}");
    let arr1 = ['a', 'b', 'c'];
    let arr2: [_; 3] = [false; 3];
    println!("{arr1:?} {arr2:?}");

    let arr3 = arr0;  // Copy
    println!("{arr0:?} {arr3:?}");
}