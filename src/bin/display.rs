use std::fmt;

fn main() {
    println!("hello world");
    println!("{}", "hello world"); // display    需要实现 std::fmt::Display trait
    println!("{:?}", "hello world"); // debug 格式，需要实现 std::fmt::Debug  trait
    println!("{:#?}", "hello world"); // 美化输出
    println!(
        "{1:?} {0:#?} {actor} {actor:?} {0}",
        "hello",
        "hi",
        actor = "Actor"
    );

    println!("\n--------------------");
    // if used in a formatting string, curly braces are escaped with `{{` and `}}`  not `\{`
    println!("{{}}");
    println!("{{"); // 必须是偶数组
    println!("}}");
    println!("{{{{"); // 转义输出 {{
    println!("{{{{}}}}"); // 转义输出 {{}}

    println!("\n--------------------");
    let name = "Peter";
    let age = 18;
    let peter = Person { name, age };
    let peter2 = Person { age, name };
    println!("{:#?} \n{:?}", peter, peter2);
    println!("{}", peter);

    println!("\n--------------------");
    let complex = Complex {
        real: 3.3,
        image: 5.3,
    };
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}

// #[warn(dead_code)]  默认 未使用到的定义，会警告
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// 手动实现std::fmt::Display
impl fmt::Display for Person<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The person name is {}, age is {}", self.name, self.age)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    image: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.image)
    }
}
