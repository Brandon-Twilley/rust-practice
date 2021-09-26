mod linked_list;
use std::{cell::RefCell, rc::Rc};

#[allow(while_true)]
fn main() {
    let n: Vec<Rc<RefCell<linked_list::Node>>> = vec![
        linked_list::new_node("This".to_string()),
        linked_list::new_node("is".to_string()),
        linked_list::new_node("a".to_string()),
        linked_list::new_node("linked".to_string()),
        linked_list::new_node("list".to_string()),
    ];
    let f = Box::new(
        |n1: Rc<RefCell<linked_list::Node>>, n2: Rc<RefCell<linked_list::Node>>| {
            linked_list::join(n1, n2);
        },
    );
    f(n[0].clone(), n[1].clone());
    f(n[1].clone(), n[2].clone());
    f(n[2].clone(), n[3].clone());
    f(n[3].clone(), n[4].clone());

    let mut r = n[0].clone();
    while true {
        println!("{}", r.borrow().text);
        let a = linked_list::next(r);
        match a {
            Some(a) => {
                r = a;
            }
            None => break,
        }
    }
}
