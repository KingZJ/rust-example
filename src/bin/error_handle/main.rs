mod err_result;
mod unpack;

fn main() {
    give_princess("teddy bear");
    // give_princess("snake");

    give_commoner(Some("teddy bear"));
    give_commoner(Some("snake"));
    give_commoner(None);

    give_princess1(Some("teddy bear"));
    // give_princess1(Some("snake"));
    // give_princess1(None);

    println!("\n============");
    let p = unpack::Person {
        job: Some(unpack::Job {
            phone: Some(unpack::Phone {
                area_code: Some(61),
                number: 13988889999,
            }),
        }),
    };
    println!("p area code: {:?}", p.work_phone_area_code());

    println!("10 * 2 = {}", multiply("10", "2"));
    println!("10 * 2 = {:?}", mul2("10", "2"));

    // 以下都会报错，无法解析为数字
    // println!("10 * 2 = {}", multiply("10.09", "2"));
    // println!("10 * 2 = {}", multiply("1t", "2"));
    // println!("10 * 2 = {}", multiply("t1", "2"));
    println!("10 * 2 = {:?}", mul2("tt", "2"));

    println!("\n==========");
    err_result::result_map_test();
    err_result::test2();
    err_result::test3();
    err_result::test4();
}

fn multiply(s1: &str, s2: &str) -> i32 {
    // parse 返回 Result
    let first_num = s1.parse::<i32>().unwrap();
    let second_num = s2.parse::<i32>().unwrap();

    first_num * second_num
}

fn mul2(s1: &str, s2: &str) -> Result<i32, std::num::ParseIntError> {
    s1.parse::<i32>()
        .and_then(|first_num| s2.parse::<i32>().map(|second_num| first_num * second_num))
}

fn give_princess(gift: &str) {
    if gift == "snake" {
        panic!("AAaaa!!!!!!!!");
    }

    println!("I love {}!!!!", gift);
}

fn give_commoner(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("throwing the snake in a fire"),
        Some(o) => println!("{}? How nice.", o),
        None => println!("no gift"),
    }
}

fn give_princess1(gift: Option<&str>) {
    // let inside = gift.unwrap();  // Some(v) 中的 v 返回， None 则报错
    let inside = gift.expect("a gift is must"); // Some(v) 中的 v 返回， None 则返回panic expect
    if inside == "snake" {
        panic!("AAaaa!!!!!!!!");
    }

    println!("I love {}!!!!", inside);
}
