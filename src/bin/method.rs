fn main() {
    let rec = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };
    println!("rec perimeter: {}, area: {}", rec.perimeter(), rec.area());

    rec.panic();
    Rectangle::test();
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2f64 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn test() {
        println!("static method in impl Rectangle, by Rectangle::test()");
    }

    fn panic(self) {
        let p1 = self.p1;
        // let Point {x: x2, y: y2} = self.p2;
        println!("{:?}", p1);
        println!("rectangle variable moved");
    }
}
