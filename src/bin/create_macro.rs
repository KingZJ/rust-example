macro_rules! say_hello {
    () => {
        println!("Hello!!!!!");
    };
}

macro_rules! create_fn {
    ($fn_name:ident) => {
        fn $fn_name() {
            println!("You called {:?}", stringify!($fn_name))
        }
    };
}

create_fn!(foo);
create_fn!(bar);

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression)
    };
}

macro_rules! find_min {
    ($x:expr) => ($x);

    //  `$x` 后至少一个 `$y`
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y),+))
    )
}

/// 调用宏 在宏定义后，在前面会找不到，与类型和函数不同
fn main() {
    say_hello!();
    foo();
    bar();

    print_result!(1 + 2);

    println!("min: {}", find_min!(1));
    println!("min: {}", find_min!(1, 2, 0));
}
