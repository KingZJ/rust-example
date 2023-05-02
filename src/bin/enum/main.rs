mod node;
mod page;

fn main() {
    let pressed = page::WebEvent::KeyPress('x');
    let pasted = page::WebEvent::Paste("text text text".to_owned());
    let click = page::WebEvent::Click { x: 20, y: 30 };
    let load = page::WebEvent::PageLoad;
    let unload = page::WebEvent::PageUnload;

    page::inspect(load);
    page::inspect(pressed);
    page::inspect(pasted);
    page::inspect(click);
    page::inspect(unload);

    println!("===========链表===========");
    let mut list = node::Node::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("list({}) length is {}", list.stringify(), list.len());
    println!("{:?}", list);
}
