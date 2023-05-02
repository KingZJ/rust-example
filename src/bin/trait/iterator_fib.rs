pub struct Fibonacci {
    cur: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.cur;
        let next = self.cur + self.next;

        self.cur = self.next;
        self.next = next;

        Some(ret)
    }
}

impl Fibonacci {
    pub fn fibonacci() -> Self {
        Self { cur: 1, next: 1 }
    }
}

pub fn iter_test() {
    // 区间是一个 Iterator
    let mut seq = 0..3;
    println!("> {:?}", seq.next());
    println!("> {:?}", seq.next());
    println!("> {:?}", seq.next());
    println!("> {:?}", seq.next());
    println!("> {:?}", seq.next());

    for i in Fibonacci::fibonacci().take(4) {
        println!("> {}", i);
    }
}
