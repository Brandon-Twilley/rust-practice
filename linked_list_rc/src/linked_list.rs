use std::{cell::RefCell, rc::Rc, string::String};
pub struct Node {
    pub before: Option<Rc<RefCell<Node>>>,
    pub text: String,
    pub after: Option<Rc<RefCell<Node>>>,
}

pub fn new_node(s: String) -> Rc<RefCell<Node>> {
    Rc::new(RefCell::new(Node {
        before: None,
        text: s,
        after: None,
    }))
}
pub fn join(bef: Rc<RefCell<Node>>, aft: Rc<RefCell<Node>>) {
    bef.borrow_mut().after = Some(aft.clone());
    aft.borrow_mut().before = Some(bef.clone());
}

pub fn next(r: Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
    match &r.borrow().after {
        Some(c) => Some(c.clone()),
        None => None,
    }
}
