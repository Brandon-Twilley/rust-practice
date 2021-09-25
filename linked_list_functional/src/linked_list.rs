#[derive(Debug)]
pub struct Node {
    pub before: *mut Node,
    pub text: std::string::String,
    pub after: *mut Node,
}

pub struct Runner {
    pub current_node: *mut Node,
}

pub fn create_node(txt: std::string::String) -> Node {
    Node {
        before: std::ptr::null_mut(),
        text: txt,
        after: std::ptr::null_mut(),
    }
}

pub fn create_runner(first_node: *mut Node) -> Runner {
    Runner {
        current_node: first_node,
    }
}

pub unsafe fn join_nodes(before: *mut Node, after: *mut Node) -> *mut Node {
    (*before).after = after;
    (*after).before = before;
    return after;
}

pub unsafe fn step(r: *mut Runner) -> bool {
    if (*(*r).current_node).after == std::ptr::null_mut() {
        return false;
    } else {
        (*r).current_node = (*(*r).current_node).after;
        return true;
    }
}
