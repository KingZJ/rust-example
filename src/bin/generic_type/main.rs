/// 泛型 类型参数，使用尖括号和大驼峰命名 <Args>   <T>

mod const_generic;
mod normal_generics;

fn main() {
    println!(".......");

    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(90));

    generic_fn::<char>(SGen('c')); // 显式调用指定类型
    generic_fn(SGen('a')); // 隐式

    let x = A;
    let y = S(x);
    let xz = SGen(A);
    let yz = SGen(y);
    let z = SGen(1f64);

    xz.test1();
    xz.test_t();
    yz.test1();
    yz.test_t();
    z.test_t();

    const_generic::test();
}

#[derive(Debug)]
struct A;

#[derive(Debug)]
struct S(A); // A 代表类型

#[derive(Debug)]
struct SGen<T>(T); // T 代表泛型

fn reg_fn(_s: S) {}

/// 接收类型 SGen<A>  其中 A为结构体 A，不是泛型
fn gen_spec_t(_s: SGen<A>) {}

///
fn gen_spec_i32(_s: SGen<i32>) {}

/// 接收类型 SGen<T> 的参数， 关于 T 泛型函数
fn generic_fn<T>(_s: SGen<T>) {}

impl SGen<A> {
    fn test1(&self) {
        println!("SGen(A) test....")
    }
}

impl SGen<S>
where
    S: std::fmt::Debug,
{
    fn test1(&self) {
        println!("SGen(S) test....")
    }
}

impl<T: std::fmt::Debug> SGen<T> {
    fn test_t(&self) {
        println!("{:?} test....", self)
    }
}

/// 泛型 trait
trait Contains1<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
}

/// 关联类型实现
trait Contains2 {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
}
