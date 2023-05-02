pub mod lifetime;
pub mod ownership;

// pub use crate::lifetime;

fn main() {
    println!("ownership partial moved or borrowed");
    ownership::partial_moved();

    println!("\nownership moved or borrowed");
    ownership::borrowed_or_moved();

    println!("\nlifetime");
    lifetime::lifetime_apply();
}
