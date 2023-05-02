/// 单向链表 或者 节点定义
use Node::*;

#[derive(Debug)]
pub enum Node {
    Cons(i32, Box<Node>),
    Nil,
}

impl Node {
    pub fn new() -> Self {
        Nil
    }

    pub fn prepend(self, ele: i32) -> Node {
        Cons(ele, Box::new(self))
    }

    pub fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    pub fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
            Nil => format!("Nil"),
        }
    }
}
