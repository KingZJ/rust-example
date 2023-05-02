fn main() {
    println!("lifetime test");
    test1();

    test2();
    test3();
}

struct User<'a> {
    first_name: &'static str,
    last_name: &'a str,
}

impl<'b> User<'b> {
    fn get_first_name(&self) -> &str {
        self.first_name
    }

    fn get_last_name(&self) -> &str {
        self.last_name
    }
}

fn test1() {
    let a;

    {
        let b = 10;
        // a = &b;
        a = b;
    }

    println!("{}", a);
}

fn test2() {
    let s1 = "Hi";
    let longest;
    {
        let s2 = "Hello";
        longest = find_longest(s1, s2);
    }

    println!("{}", longest);
}

fn test3() {
    let s1 = String::from("Hi"); // 字符串字面量
                                 // let s1 = "Hi";
    let longest;
    {
        // let s2 = String::from("Hello");
        let s2 = "Hello";
        longest = find_longest(s2, &s1); // s2 的生命周期短
    }

    println!("{}", longest);
}

fn find_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
