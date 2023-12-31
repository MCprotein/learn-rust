use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    /*
        각 노드에 직접적으로 액세스할 수 있는 변수들의 소유권을 공유하기위해 Rc 사용
        어떤 노드를 다른 노드의 자식으로 수정하기 위해 RefCell 사용
        parent에도 Rc를 사용하면 leaf.parent가 branch를 가리키고, branch.children이 leaf를 가리켜 참조 사이클이 발생한다.
        parent에 Weak를 사용했으므로 부모 노드를 참조할 수 있지만 소유하지는 않는다.
    */
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

pub fn tree_node() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });

    // downgrade: Rc타입에서 Weak 타입 얻기 위해 호출
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch)
    );

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    // upgrade: Weak타입에서 Option<Rc<T>> 얻기 위해 호출
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
