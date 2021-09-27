use std::{cell::RefCell, option::Option, rc::Rc};

#[derive(Debug)]
pub struct Node<T> {
    object: Option<Rc<RefCell<T>>>,
    next: Vec<Option<Rc<RefCell<Node<T>>>>>,
}

pub fn new_node<T>(obj: Rc<RefCell<T>>) -> Rc<RefCell<Node<T>>> {
    let mut n: Vec<Option<Rc<RefCell<Node<T>>>>> = vec![];
    for _ in 0..52 {
        n.push(None)
    }
    Rc::new(RefCell::new(Node {
        object: Some(obj.clone()),
        next: n,
    }))
}

pub fn empty_node<T>() -> Rc<RefCell<Node<T>>> {
    let mut n: Vec<Option<Rc<RefCell<Node<T>>>>> = vec![];
    for _ in 0..52 {
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

impl<T: std::fmt::Debug> Node<T> {
    pub fn push(&mut self, s: String, obj: Rc<RefCell<T>>) -> bool {
        match s.get(..1) {
            Some(a) => match s.get(1..) {
                Some(b) => self.char_switch_push(a, b, obj),
                None => self.char_switch_push(a, "", obj),
            },
            None => self.set_object(obj),
        }
    }
    // Pushes the given node onto the vector list
    fn char_switch_push(&mut self, index: &str, rest: &str, obj: Rc<RefCell<T>>) -> bool {
        match index.chars().nth(0).unwrap() {
            //If our index character is a capital letter
            'A'..='Z' => self.normalize_char_push(index, rest, 65, obj),
            //If our index character is a capital letter
            'a'..='z' => self.normalize_char_push(index, rest, 71, obj),
            _ => false,
        }
    }
    fn normalize_char_push(
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
            None => match rest.len() {
                0 => {
                    self.next[normalized].replace(new_node(obj.clone()));
                    true
                }
                _ => {
                    self.next[normalized].replace(empty_node());
                    self.normalize_char_push(index, rest, normalize_num, obj)
                }
            },
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
    pub fn get(&self, s: String) -> (ERROR, Option<Rc<RefCell<T>>>) {
        match s.get(..1) {
            Some(a) => match s.get(1..) {
                Some(b) => self.char_switch_get(a, b),
                None => self.char_switch_get(a, ""),
            },
            None => (ERROR::NO_INDEX, None),
        }
    }
    fn char_switch_get(&self, index: &str, rest: &str) -> (ERROR, Option<Rc<RefCell<T>>>) {
        match index.chars().nth(0).unwrap() {
            //If our index character is a capital letter
            'A'..='Z' => self.normalize_char_get(index, rest, 65),
            //If our index character is a capital letter
            'a'..='z' => self.normalize_char_get(index, rest, 71),
            _ => (ERROR::UNSUPPORTED_CHARS, None),
        }
    }
    fn normalize_char_get(
        &self,
        index: &str,
        rest: &str,
        normalize_num: usize,
    ) -> (ERROR, Option<Rc<RefCell<T>>>) {
        let normalized = (index.chars().nth(0).unwrap() as usize) - normalize_num;
        println!(" {}", normalized);
        match self.next[normalized].clone() {
            //If our selected node already exists
            Some(node) => match rest.len() {
                0 => (ERROR::NONE, node.borrow().object.clone()),
                _ => node.borrow().get(rest.to_string()),
            },
            // if our selected node doesn't already exist
            None => (ERROR::NULL_POINTER, None),
        }
    }
}

#[derive(Debug)]
pub enum ERROR {
    NONE,
    NO_INDEX,
    UNSUPPORTED_CHARS,
    NULL_POINTER,
    POINTER_SET,
}
