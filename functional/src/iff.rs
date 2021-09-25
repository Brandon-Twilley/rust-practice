pub fn iff<T: 'static>(branch: Vec<Box<dyn Fn() -> T>>) -> Box<dyn Fn(bool) -> T> {
    Box::new(move |cond: bool| branch[cond as usize]())
}

pub struct Closure {}

impl Closure {
    pub fn true_branch() -> bool {
        true
    }
    pub fn false_branch() -> bool {
        false
    }
}
