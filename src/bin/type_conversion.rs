use std::convert::{From, Into};
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

type NanoSecond = u64;

fn main() {
    let decimal = 65.789;

    // 原生类型：i8 u8 等 char bool f32 f63  单元()空元组  符合类型-数组、切片
    // 原生类型转换需要通过 as 显示类型转换，不支持隐式转换
    let i = decimal as u8;
    let c = i as char;
    println!("casting: {} -> {} -> {}", decimal, i, c);
    // println!("128 as a u8 is: {}", 128 as i8);

    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 1;
    let f = 1.0;
    println!("size of x in bytes: {}", std::mem::size_of_val(&x));
    println!("size of y in bytes: {}", std::mem::size_of_val(&y));
    println!("size of z in bytes: {}", std::mem::size_of_val(&z));
    println!("size of i in bytes: {}", std::mem::size_of_val(&i));
    println!("size of f in bytes: {}", std::mem::size_of_val(&f));

    // 类型别名 必须需准守大驼峰命名规则，
    // 别名并不提供类型安全，别名并不是新的类型
    let ns: NanoSecond = 6u64;
    println!("{}", ns);

    println!("=============");
    let val: i32 = 32;
    // println!("i32 into Number: {:?}", val.into::<Number>());
    let num = Number::from(val);
    println!("{:?} \n{0:?}", num);
    println!("{}", <i32 as Into<Number>>::into(val));
    // println!("{}", val.into());

    println!("{}", i32::from(num));
    // println!("{}", <Number as Into<i32>>::into(num));

    let num: Result<Number, _> = "10".parse();
    println!(
        "{:?} {:?} {:?}",
        "32".parse::<Number>(),
        Number::from_str("5"),
        num
    );

    println!(
        "from Inter to i32: {:?} -> {:?}",
        Inter(10),
        <Inter as Into<i32>>::into(Inter(10))
    );
}

#[derive(Debug)]
struct Inter(i32);

impl From<Inter> for i32 {
    fn from(item: Inter) -> Self {
        item.0
    }
}

#[derive(Debug)]
struct Number {
    val: i32,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ val: {} }}", self.val)
    }
}

/// 实现了 From trait，就会自动实现 Into trait
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Self { val: item }
    }
}

/// 字符串解析为 Number 类型
impl FromStr for Number {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val: i32 = s.parse()?;
        Ok(Self { val })
    }
}

/* 与From trait 同时存在时 会报错，原因暂时不知到 TODO */
// impl TryFrom<i32> for Number {
//     type Error = String;
//     fn try_from(item: i32) -> Result<Self, Self::Error> {
//         if item % 2 == 0 {
//             Ok(Number {val: item})
//         } else {
//             Err("error".to_owned())
//         }
//     }
// }

impl From<Number> for i32 {
    fn from(item: Number) -> Self {
        item.val
    }
}
