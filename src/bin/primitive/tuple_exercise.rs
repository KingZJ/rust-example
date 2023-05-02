pub fn test() {
    println!("\n........tuple type exercise......");
    // destructuring assignments
    let (x, y); // declare
    (x, ..) = (3, 4); // assign
    [.., y] = [1, 2];
    println!("Success! x:{x}, y:{y}");

    // tuple
    let (mut x, y): (i32, i32) = (1, 2);
    x += 2;
    println!("Success! x:{x}, y:{y}");
}
