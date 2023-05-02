macro_rules! say {
    () => {
        println!("hello")
    };
}
static mut COUNTER: u32 = 0;
static HELLO: &'static str = "Hello";
const NUM: u32 = 0;
fn main() {
    println!("Hello, world!");
    say!();

    println!("{:?} {:?}", vb, ID::right);

    // let x = Some(Some("owned".to_owned()));   // Some(y)  需要修改为 Some(ref y)
    let x = Some(Some(5));
    if let Some(y) = x {
        println!("x: {:?}, y: {:?}", x, y);
    }
    // println!("x: {:?}, y:{:?}", x, y);
    println!("x: {:?}", x);

    let mut num = 5;
    let r1 = &mut num;
    // let r2 = &num;
    println!("{:?}", r1);

    let guess = "tt";
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("not a u32 number"),
    };

    /* // 等价于如下代码
    let guess = "tt";
    let mut num: u32;
    match guess.trim().parse() {
        Ok(x) => num = x,
        Err(_) => continue,
    }*/

    println!("{:?}", guess);

    assert!(1 == 2);
}

#[derive(Debug)]
struct vb;

#[derive(Debug)]
enum ID {
    right,
    wrong,
}
