use std::rc::Rc;

pub struct Counter<T> {
    pub vec: Vec<Rc<T>>,
}

pub fn create_counter<T>(v: Vec<Rc<T>>) -> Counter<T> {
    Counter { vec: v }
}

impl<T: std::cmp::PartialEq + std::fmt::Display + std::fmt::Debug> Counter<T> {
    pub fn count_all(&self) -> usize {
        return self.vec.iter().count();
    }

    pub fn count_match(&self, _check: T) -> usize {
        self.vec
            .iter()
            .filter(|vector_element| _check == *Rc::as_ref(&vector_element))
            .count()
    }

    pub fn count_not_match(&self, _check: T) -> usize {
        self.vec
            .iter()
            .filter(|vector_element| _check != *Rc::as_ref(&vector_element))
            .count()
    }
}
