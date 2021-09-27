use std::{cell::RefCell, option::Option, rc::Rc};

#[derive(Debug)]
pub struct Node<T> {
    object: Option<Rc<RefCell<T>>>,
    next: Vec<Option<Rc<RefCell<Node<T>>>>>,
}

pub fn new_node<T>(obj: Rc<RefCell<T>>) -> Rc<RefCell<Node<T>>> {
    let mut n: Vec<Option<Rc<RefCell<Node<T>>>>> = vec![];
    for _ in 0..51 {
        n.push(None)
    }
    Rc::new(RefCell::new(Node {
        object: Some(obj.clone()),
        next: n,
    }))
}

pub fn empty_node<T>() -> Rc<RefCell<Node<T>>> {
    let mut n: Vec<Option<Rc<RefCell<Node<T>>>>> = vec![];
    for _ in 0..51 {
        n.push(None)
    }
    Rc::new(RefCell::new(Node {
        object: None,
        next: n,
    }))
}

impl<T> Clone for Node<T> {
    fn clone(&self) -> Node<T> {
        Node {
            object: self.object.clone(),
            next: self.next.clone(),
        }
    }
}

impl<T> Node<T> {
    pub fn push(&mut self, s: String, obj: Rc<RefCell<T>>) -> bool {
        match s.get(..1) {
            Some(a) => match s.get(1..) {
                Some(b) => self.char_switch(a, b, obj),
                None => self.char_switch(a, "", obj),
            },
            None => self.set_object(obj),
        }
    }
    // Pushes the given node onto the vector list
    fn char_switch(&mut self, index: &str, rest: &str, obj: Rc<RefCell<T>>) -> bool {
        match index.chars().nth(0).unwrap() {
            //If our index character is a capital letter
            'A'..='Z' => self.normalize_char(index, rest, 65, obj),
            //If our index character is a capital letter
            'a'..='z' => self.normalize_char(index, rest, 71, obj),
            _ => false,
        }
    }
    fn normalize_char(
        &mut self,
        index: &str,
        rest: &str,
        normalize_num: usize,
        obj: Rc<RefCell<T>>,
    ) -> bool {
        let normalized = (index.chars().nth(0).unwrap() as usize) - normalize_num;
        match self.next[normalized].clone() {
            //If our selected node already exists
            Some(node) => node.borrow_mut().push(rest.to_string(), obj.clone()),
            // if our selected node doesn't already exist
            None => {
                    self.set_index(normalized, new_node(obj.clone()));
                    self.normalize_char(index, rest, normalize_num, obj.clone())
                }
            // None => match rest.len() {
            //     0 => self.set_index(normalized, new_node(obj.clone())),
            //     _ => false,
            // },
        }
    }
    fn set_index(&mut self, index: usize, node_ptr: Rc<RefCell<Node<T>>>) -> bool {
        match &self.next[index] {
            Some(_) => false,
            None => {
                self.next[index] = Some(node_ptr);
                true
            }
        }
    }
    fn set_object(&mut self, obj: Rc<RefCell<T>>) -> bool {
        match &self.object {
            Some(_) => false,
            None => {
                self.object = Some(obj.clone());
                true
            }
        }
    }
    fn get_object(&mut self) -> (bool, Option<Rc<RefCell<T>>>) {
        match &self.object {
            Some(a) => (true, Some(a.clone())),
            None => (false, None),
        }
    }
}
