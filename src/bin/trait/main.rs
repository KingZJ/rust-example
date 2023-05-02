mod advanced_trait;
mod iterator_fib;
mod trait_object;

fn main() {
    let mut dolly: Sheep = Sheep::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();

    let rand_num = 0.53;
    println!("{:?}", rand_animal(rand_num).noise());

    println!("\n in main");
    let _c1 = Cow::new("c1");
    let _c2 = Cow::new("c2");
    println!("== end");

    iterator_fib::iter_test();
    trait_object::main();
    advanced_trait::test();
}

fn rand_animal(rand_num: f64) -> Box<dyn Animal> {
    if rand_num < 0.5 {
        Box::new(Sheep::default())
    } else {
        Box::new(Cow::default())
    }
}

/// 派生 通过 derive 属性 自动部署 trait
#[derive(Debug, Default)]
struct Sheep {
    naked: bool,
    name: &'static str,
}

#[derive(Debug, Default)]
struct Cow {
    name: &'static str,
}

impl Drop for Cow {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

trait Animal {
    // fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    /// 默认实现的方法定义
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name())
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }

    fn new(name: &'static str) -> Self {
        Self {
            name: name,
            naked: false,
        }
    }
}

impl Animal for Sheep {
    // fn new(name: &'static str) -> Self {
    //     Self {
    //         name: name,
    //         naked: false,
    //     }
    // }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "bah"
        } else {
            "bah!"
        }
    }

    /// 默认 trait 方法可以重载
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

impl Cow {
    fn new(name: &'static str) -> Self {
        Self { name: name }
    }
}

impl Animal for Cow {
    //    fn new(name: &'static str) -> Self {
    //         Self {
    //             name: name,
    //         }
    //     }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        "bah~"
    }
}
