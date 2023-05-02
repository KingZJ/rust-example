#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
/// nightly-x86_64-apple-darwin need in macos
/// rustup toolchain install nightly-x86_64-apple-darwin
/// rustup show 查看是否安装成功
/// cargo +nightly run --bin mem_cost


use std::mem::size_of;

fn check_size<T>(_val: T)
where
    Assert<{ size_of::<T>() < 768 }>: IsTrue,
{
    //...
}

// Fix the errors in main.
fn main() {
    println!("`[&str;10]` size: {}", size_of::<[&str; 10]>());
    println!("`String` size: {}", size_of::<[String; 10]>());
    println!("`&str` size: {}", size_of::<&str>());  // &str {ptr, len}
    println!("`String` size: {}", size_of::<String>());
    println!("`char` size: {}", size_of::<char>());
    println!("`&char` size: {}", size_of::<&char>());  // ptr: usize

    check_size([0u8; 767]); 
    check_size([0i32; 191]);
    check_size(["hello你好"; 47]); // Size of &str ?
    check_size([(); 31].map(|_| "hello你好".to_string()));  // Size of String?
    check_size(['中'; 191]); // Size of char ?

    println!("Success!");
}



pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}