mod linked_list;

#[allow(while_true)]
fn main() {
    let n1: *mut linked_list::Node = &mut linked_list::create_node(String::from("I"));
    let n2: *mut linked_list::Node = &mut linked_list::create_node(String::from("am"));
    let n3: *mut linked_list::Node = &mut linked_list::create_node(String::from("the"));
    let n4: *mut linked_list::Node = &mut linked_list::create_node(String::from("eggman"));
    let n5: *mut linked_list::Node = &mut linked_list::create_node(String::from("koo"));
    let n6: *mut linked_list::Node = &mut linked_list::create_node(String::from("koo"));
    let n7: *mut linked_list::Node = &mut linked_list::create_node(String::from("kachoo"));

    unsafe {
        linked_list::join_nodes(n1, n2);
        linked_list::join_nodes(n2, n3);
        linked_list::join_nodes(n3, n4);
        linked_list::join_nodes(n4, n5);
        linked_list::join_nodes(n5, n6);
        linked_list::join_nodes(n6, n7);
    }

    let r = &mut linked_list::create_runner(n1);
    let mut next: bool = true;
    while true {
        unsafe {
            println!("{:?}", (*(*r).current_node).text);
        }
        if next == false {
            break;
        } else {
            unsafe {
                next = linked_list::step(r);
            }
        }
    }
}
