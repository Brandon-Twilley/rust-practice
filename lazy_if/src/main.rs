use std::lazy::Lazy;
mod lazy_if;

#[feature(once_cell)]
fn main() {
    fn iff<T>(cond: Vec<*const Lazy<T>>) -> *const Lazy<T> {
        cond[0]
    }
}
