trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

// IMPLEMENT below with generics.
fn static_dispatch<T:Foo>(foo: T) {
    println!("{}", foo.method());
}

// Implement below with trait objects.
fn dynamic_dispatch(foo: &dyn Foo) {
    println!("{}", foo.method());
}

pub fn main() {
    println!("\n ..... trait object exercise ......");
    let x = 5u8;
    let y = "Hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);

    println!("Success!");
}
