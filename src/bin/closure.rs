/// 名称：闭包函数 | 匿名函数 | lambda   作用：函数式编程
fn main() {
    let one = || 1; // || 代替函数中 ()
    println!("{} {}", f1(), one());

    // 输入输出类型自动推导
    // let sub = |a: i32, b:i32| -> i32 {a - b}
    let sub = |a, b| a - b;
    println!("{} + {} = {} \n{0} - {1} = {}", 2, 3, add(2, 3), sub(2, 3));

    println!("\n闭包捕获外部变量三种方式[T, &T, &mut T]");
    capture_variables();
    closure_type();

    println!("\n输出返回闭包");
    return_closure()();

    println!("\n 闭包应用练习，迭代器");
    test_iter();
}

fn test_iter() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![10, 20, 30];
    println!("2 in vector {:?}: {}", vec1, vec1.iter().any(|&x| x == 2));
    println!(
        "2 in vector {:?}: {:?}",
        vec1,
        vec1.iter().find(|&&x| x == 2)
    );
    // println!("2 in vector : {}", vec2.into_iter().any(|x| x == 2));
    print!("find 2 in {:?}: ", vec2);
    println!("{:?}", vec2.into_iter().find(|&x| x == 2));

    let arr1 = [1, 2, 3];
    let arr2 = [10, 20, 30];
    println!("2 in array {:?}: {}", arr1, arr1.iter().any(|&x| x == 2));
    println!(
        "2 in array {:?}: {}",
        arr2,
        arr2.into_iter().any(|x| x == 2)
    );

    let arr3 = arr2; // 未发生 所有权转移  验证 primitive type 都实现了Copy trait
    println!("after arr3=arr2, arr3: {:?} arr2: {:?}", arr3, arr2);

    let a = (1, 2);
    let b = a;
    println!("{:?} {:?}", a, b);

    let s1 = "s1";
    let s2 = s1;
    println!("{} {}", s1, s2);
}

fn return_closure() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a : {}", text)
}

fn closure_type() {
    use std::mem;

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let dirty = || {
        println!("I said {}", greeting); // Fn  &T 借用捕获

        farewell.push_str("!!"); // FnMut &mut T 可变借用捕获
        println!("Then I screamed {}", farewell);

        mem::drop(farewell); // FnOnce T 值捕获
    };

    apply(dirty);

    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
}

fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

fn capture_variables() {
    let color = String::from("green");
    let my_print = || println!("color: {}", color);

    my_print(); //此时捕获是 &color
    let _re_borrow = &color;
    my_print();
    let _color_moved = color; // color 被移动到其他变量，再次调用 my_print() 会报错
                              // my_print();

    fn_inc();
}

fn fn_inc() {
    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("count: {}", count);
    };
    inc();
    // let _re_borrow = &count;   // 后续再次调用 inc() 会报错
    inc();
    let _re_borrow = &count;
    println!("{}", _re_borrow);
    let re_mut_borrow = &mut count; // 第二次可变借用
                                    // inc();  // 第一次可变借用 则与第二次可变借用冲突导致报错
    *re_mut_borrow += 1;
    println!("第二次可变借用后：{}", count);
    // println!("{}", _re_borrow);
    let _count_moved = count;
}

fn f1() -> i32 {
    1
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

// 不支持重载
// fn add(x: f32, y: f32) -> f32{
//     x + y
// }
