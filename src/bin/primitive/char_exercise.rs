use std::mem::size_of_val;

pub fn test() {
    println!("\n........char type exercise......");
    println!("size of char:{}", size_of_val(&'c'));
    println!("{}", size_of_val(&'中'));
    println!("{}", '中' as u32);
    println!("{}", '💗');
    println!("{}", '💗' as u32);
    println!("{}", "🀄️");
    println!("{}", size_of_val("🀄️"));
    println!("{}", size_of_val("中"));
    println!("size of unit `()`:{}", size_of_val(&()));
    println!("size of bool: {}", size_of_val(&false));
}
