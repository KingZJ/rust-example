// bin目录下，多文件的二进制执行必须是main 命名文件
//cargo run --bin struct 可执行
mod color;
mod list;

fn main() {
    println!("in struct");

    let v = list::List(vec![1, 2, 3]);
    println!("{} \n{0:?}", v);

    let c = color::Color {
        red: 128,
        green: 255,
        blue: 90,
    };
    let c2 = color::Color { red: 255, ..c };
    println!("{} \n{:?}", c, c2);
    dbg!(c2.blue * 2);
}
