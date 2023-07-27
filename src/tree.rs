pub struct Tree {
    pub root: RefCell<Rc<Node>>,
}

pub struct Node {
    pub children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>.
    name: String,
    filesize: u32,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            root: RefCell::new(Rc::new(Node {
                children: RefCell::new(vec![]),
                parent: RefCell::new(Default::default()),
            }))
        }
    }
    pub fn make(&self) -> Rc<Node> {
        Rc::new(Node{
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        })
    }
    pub fn make_with(&self, 
              children:RefCell<Vec<Rc<Node>>>) -> Rc<Node> {
        let rc =  Rc::new(Node {
            children,
            parent: RefCell::new(Default::default()),
        });
        for ch in rc.children.borrow().iter() {
            *ch.parent.borrow_mut() = Rc::downgrade(&rc);
        }
        return rc;
    }
    pub fn set_root(&self, node:&Rc<Node>) {
        *self.root.borrow_mut() = Rc::clone(node);
    }
    pub fn append(&self, parent:&Rc<Node>, child:&Rc<Node>) {
        parent.children.borrow_mut().push(Rc::clone(child));
        *child.parent.borrow_mut() = Rc::downgrade(parent);
    }
}
