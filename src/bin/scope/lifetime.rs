use std::fmt::Debug;

/// static 声明的常量具有 'static 生命周期，最长的生命周期
/// string 字面量 拥有 &'static str 类型 eg: "hello"
static NUM: i32 = 18;

pub fn test() {
    println!("local mod no in main file, include other's ");
}

pub fn lifetime_apply() {
    let x = 7;
    let ref_x = MyRef(&x);

    print_ref(&ref_x);
    print(ref_x);

    let b = Borrowed::default();
    println!("b is: {:?} {}", b, b.x);
}

#[derive(Debug)]
struct MyRef<'a, T: 'a>(&'a T);

fn print<T>(t: T)
where
    T: Debug,
{
    println!("`print`: t is {:?}", t);
}

/// 1.T: 'a 所有引用都必须比生命周期 'a活得更长
/// 2.T: Trait + 'a T 必须实现Trait 且T中所有引用都必须比 'a 活得更长
fn print_ref<'a, T>(t: &'a T)
where
    T: Debug + 'a,
{
    println!("`print_ref`: t is {:?}", t);
}

#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self { x: &10 }
    }
}
