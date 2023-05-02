pub fn test() {
    println!("\n......const generic.....");
    
    bar::<usize, 8>();
}

fn foo<const N: usize>() {}


// A standalone const parameter.
// A literal (i.e. an integer, bool, or character).
// A concrete constant expression (enclosed by {}), involving no generic parameters
fn bar<T, const M: usize>() {
    foo::<M>(); // Okay: `M` is a const parameter
    foo::<2021>(); // Okay: `2021` is a literal
    foo::<{20 * 100 + 20 * 10 + 1}>(); // Okay: const expression contains no generic parameters
    
    // foo::<{ M + 1 }>(); // Error: const expression contains the generic parameter `M`
    // foo::<{ std::mem::size_of::<T>() }>(); // Error: const expression contains the generic parameter `T`
    
    let _: [u8; M]; // Okay: `M` is a const parameter
    // let _: [u8; std::mem::size_of::<T>()]; // Error: const expression contains the generic parameter `T`

    println!("Success!");
}