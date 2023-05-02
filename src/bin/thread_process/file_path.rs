use std::path::Path;

pub fn path_test() {
    let path = Path::new(".");
    let display = path.display();

    println!(
        "path: {:?} new path: {:?}",
        display,
        path.join("a").join("B")
    );
}
