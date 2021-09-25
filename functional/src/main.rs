mod iff;

fn main() {
    let branch: Vec<Box<dyn Fn() -> bool>> = vec![
        Box::new(iff::Closure::false_branch),
        Box::new(iff::Closure::true_branch),
    ];
    println!("{}", iff::iff(branch)(false));
}
