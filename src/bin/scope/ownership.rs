use crate::lifetime;

pub fn partial_moved() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;
    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);

    let t = (String::from("hello"), String::from("world"));
    let (ref s1, ref s2) = t;
    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}

pub fn borrowed_or_moved() {
    lifetime::test();
    let x = ToDrop; // `x` 为指向堆分配的指针
    println!("made a ToDrop: {:?}", x);

    match x {
        ToDrop => {
            println!("..........");
            // let _y = x;
        } // _ => println!("++++++++++"),
    }

    // 1.所有权转移，表明资源只能拥有一个所有者，这避免资源的重复释放
    // 2.资源被移动后，原来的所有者不能在被使用，这避免了悬挂指针(dangling pointer)的产生
    // 3.在进行赋值或者通过值传递函数参数时，资源所有权会发生转移,即ownership move
    // 4.原生类型(primitive type) 所有权不会转移，因为其实现了 Copy trait
    // 5.所有权转移时，数据的可变性可能发生改变
    let mut y = x;
    // println!("{:?}", x);  // 发生错误
    println!("{:?}", y);
    destroy(y);
    // println!("{:?}", y);   // 发生错误

    let mut t = (Box::new(5), 3);
    println!("init: {:?}", t);
    {
        // ref 关键字在结构 或模式匹配时，创建字段引用
        let (_, ref mut y) = t;
        *y = 33;
    }

    println!("after: {:?}", t);

    // 发生部分移动，父级变量不能在使用，但是其他部分保留
    match t {
        // `t.0` move to `a`, `t` value partially move
        // (/* ref */ a, b) => println!("matched: {:?} {:?}", a, b),
        ref t1 @ (ref a, ..) => {
            println!("partial move: {:?} {:?} {:?}", t, t1.0, a);
        } // _ => println!("++++++"),
    }
    // println!("after no ref match {:?}", t);
}

#[derive(Debug)]
struct ToDrop;

impl Drop for ToDrop {
    // &mut self <=> self: &mut Self
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn destroy(x: ToDrop) {
    println!("Destroying ToDrop that contains {:?}", x);
}
