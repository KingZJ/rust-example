use std::mem;

mod arr_type;
mod char_exercise;
mod slice_exercise;
mod str_type;
mod tuple_exercise;

fn main() {
    println!("{} {} {:e} {}", u8::MAX, 2_222_222, 2f32, 2_f32);

    println!("{:?} {:?}", (1, 2), (0,));

    // [type; length]
    let arr: [i64; 2] = [2, 7]; // declare and assign
    println!(
        "Array {:?} elements first: {}, second: {}",
        arr, arr[0], arr[1]
    );
    // 数组在栈中分配内存
    println!("Array {:?} occupy {} bytes", arr, mem::size_of_val(&arr));

    // 数组借用称为 slice  [..len]区间为左闭右开
    println!("slice from array {:?} is {:?}", arr, &arr[..1]);

    tuple_exercise::test();
    char_exercise::test();
    str_type::test();
    arr_type::test();
    slice_exercise::test();
}
