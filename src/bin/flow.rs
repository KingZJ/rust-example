fn main() {
    println!("=====选择流程=====");
    if_test();

    println!("=====循环流程=====");
    loop_test();
    fizz_buzz_while();
    fizz_buzz_for();

    println!("=====match======");
    match_test();
    tuple_match();
    struct_match();
    guard_match();
    bind_match();

    println!("======if let========");
    let mut num = Some(7);
    let letter: Option<i32> = None;
    let i_like_letter = false;

    if let Some(i) = letter {
        println!("letter matched {:?}", i);
    } else if i_like_letter {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emotion :)!");
    }

    while let Some(i) = num {
        if i > 9 {
            println!("Greater than 9, quit!");
            num = None;
        } else {
            println!("`i` is {:?}. Try again.", i);
            num = Some(i + 1);
        }
    }
}

fn if_test() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
}

fn loop_test() {
    let mut count = 0;
    loop {
        count += 1;
        if count % 3 == 0 {
            println!("three");
            continue;
        }
        println!("count: {}", count);

        if count % 5 == 0 {
            println!("OK, that's enough");
            break;
        }
    }
}

fn fizz_buzz_while() {
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzBuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("n: {}", n);
        }

        n += 1;
    }
}

fn fizz_buzz_for() {
    // 1..101  1..=100
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzBuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("n: {}", n);
        }
    }
}

fn match_test() {
    let num = 13;

    println!("Tell me about {}", num);
    match num {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let b = true;
    let bv = match b {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", b, bv);
}

fn tuple_match() {
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);

    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"), // .. 忽略其余部分
        _ => println!("It doesn't matter what they are"),
    }
}

fn struct_match() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;
    println!("a = {}, b = {}, y = {}", a, b, y);

    let Foo { y, .. } = foo;
    println!("y = {}", y);
}

fn guard_match() {
    let pair = (2, -2);
    println!("Tell me about {:?}", pair);

    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation...."),
    }
}

fn age() -> u32 {
    15
}

fn bind_match() {
    match age() {
        0 => println!("first birthday"),
        n @ 1..=12 => println!("child of age {:?}", n),
        n @ 13..=19 => println!("teen of age {:?}", n),
        n => println!("old person of age {:?}", n),
    }

    match some_number() {
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n) => println!("Not interesting... {}", n),
        _ => (),
    }
}

fn some_number() -> Option<u32> {
    Some(42)
}
