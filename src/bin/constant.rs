// 全局变量在所有其他作用域之外声明
// 常量定义 类型需要显示声明
static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;
// const LEN = 10;  报错

fn main() {
    println!("{} {} {0:?}", LANGUAGE, THRESHOLD);
}
