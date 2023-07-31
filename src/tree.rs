

mod tree {
    use std::rc::Rc;
    use std::cell::RefCell;
//    use std::rc::Weak;

/*
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
    pub fn append(&self, parent: &Rc<Node>, child: &Rc<Node>) {
        parent.children.borrow_mut().push(Rc::clone(child));
        *child.parent.borrow_mut() = Rc::downgrade(parent);
    }
}
*/

    pub struct Tree {
        pub root: RefCell<Rc<Node>>
    }

    pub struct Node {
        pub name:       String,
        pub size:       u32,
        pub children:   RefCell<Vec<Rc<Node>>>,
    }
    impl Node {
        pub fn new() -> Node {
            Node {
                name: "".to_string(),
                size: 0,
                children: RefCell::new(Vec::new()),
            }
        }
        pub fn walk(&self) {
            if self.children.borrow().is_empty() {
                println!("name: {0}, size: {1}", self.name, self.size );
            } 
            else {
                println!("dirname: {0}, size: {1}", self.name, self.size);
                for ch in self.children.borrow().iter() {
                    ch.walk();
                }
            }
        }
    }

    impl Tree {
        pub fn new() -> Tree {
            let mut node = Node::new(); 
            node.name = "/".to_string();
            Tree {
                root: RefCell::new(Rc::new(node))
            }
        }
        pub fn append(&self, parent: &Rc<Node>, child: &Rc<Node>) {
            parent.children.borrow_mut().push(child.clone());
        }

    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use std::cell::RefCell;

    use super::tree::*;
    #[test]
    fn t1() {
        let tree = Tree::new();
        {
            let child1 = Rc::new(Node {
                name: "dir1".to_string(), 
                size: 0, 
                children: RefCell::new(Vec::new()),
            }); 
            tree.append(&tree.root.borrow(), &child1);  
            assert_eq!(tree.root.borrow().children.borrow().len(), 1);

            let child2 = Rc::new(Node {
                name: "file1".to_string(), 
                size: 20, 
                children: RefCell::new(Vec::new()),
            });
            tree.append(&child1, &child2); 
            assert_eq!(child1.children.borrow().len(), 1);

            tree.append(&child1, &Rc::new(Node {
                name:"file2".to_string(), 
                size:25,
                children:RefCell::new(vec![])
            })); 
            assert_eq!(child1.children.borrow().len(), 2);

        }
        tree.root.borrow().walk();
        
        assert_eq!(5, 5);
    }
}