mod count_vals;
use std::rc::Rc;

fn main() {
    let branch: Vec<Rc<bool>> = vec![Rc::new(true), Rc::new(false), Rc::new(false), Rc::new(true)];
    let c = count_vals::create_counter::<bool>(branch);
    println!("{}", c.count_all());
    println!("{}", c.count_match(true));
    println!("{}", c.count_not_match(true));
}
