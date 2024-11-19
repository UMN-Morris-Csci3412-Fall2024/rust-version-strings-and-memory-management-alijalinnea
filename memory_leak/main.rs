use std::rc::Rc;
use std::cell::RefCell;

#[allow(dead_code)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

fn main() {
    let first = Rc::new(RefCell::new(Node { value: 1, next: None }));
    let second = Rc::new(RefCell::new(Node { value: 2, next: Some(Rc::clone(&first)) }));

    // Create a cycle
    first.borrow_mut().next = Some(Rc::clone(&second));

    println!("Memory leak created via reference counting cycle.");
}
