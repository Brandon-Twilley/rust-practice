mod linked_list;

#[allow(while_true)]
#[allow(unused_must_use)]
fn main() {
    // This is a
    let n = |s: &str| linked_list::create_node(String::from(s));
    let jn = |n1: *mut linked_list::Node, n2: *mut linked_list::Node| unsafe {
        linked_list::join_nodes(n1, n2)
    };
    const LIST_SIZE: usize = 7;

    let na: [*mut linked_list::Node; LIST_SIZE] = [
        &mut n("I"),
        &mut n("am"),
        &mut n("the"),
        &mut n("eggman"),
        &mut n("koo"),
        &mut n("koo"),
        &mut n("kachoo"),
    ];
    // let v: [*mut linked_list::Node; LIST_SIZE];
    let mut r: linked_list::Runner;

    let mut v = na
        .iter()
        .enumerate()
        .map(|(i, _)| {
            jn(
                na[if i == 0 { LIST_SIZE - 1 } else { i - 1 }],
                na[i % LIST_SIZE],
            )
        })
        .collect::<Vec<*mut linked_list::Node>>();
    unsafe {
        (*v[LIST_SIZE - 1]).after = std::ptr::null_mut();
    }

    r = linked_list::create_runner(v[0]);

    let mut next: bool = true;
    while true {
        if next == false {
            break;
        } else {
            unsafe {
                println!("{:?}", (*(r).current_node).text);
                next = linked_list::step(&mut r);
            }
        }
    }
}
