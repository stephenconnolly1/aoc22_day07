

pub mod tree {
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::rc::Weak;

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
        pub root: Rc<Node>,
    }
    enum NodeType {
        File,
        Dir,
    }
    pub struct Node {
        pub name:       String,
        pub size:       u32,
        pub parent: RefCell<Weak<Node>>,
        pub children:   RefCell<Vec<Rc<Node>>>,
    }
    impl Node {
        pub fn new() -> Node {
            Node {
                name: "".to_string(),
                size: 0,
                parent: RefCell::new(Weak::new()),
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
        pub fn get_child(&self, node_name: String) -> Rc<Node> {
            for ch in  self.children.borrow().iter() {
                if ch.name == node_name  {
                    return ch.clone();
                }
            }
            println!("node not found");
            return Rc::new (Node::new());
        }
        pub fn get_child2(&self, node_name: String) -> Rc<Node> {
            if let Some(r) =  self.children.borrow().iter().find(|&x|x.name == node_name ) {
                return r.clone();
            }
            return Rc::new (Node::new());
        }
    }

    impl Tree {
        pub fn new() -> Tree {
            let mut node = Node::new(); 
            node.name = "/".to_string();
            Tree {
                root: Rc::new(node)
            }
        }
        pub fn append(&self, parent: &Rc<Node>, child: &Rc<Node>) {
            // Add a child to the parent's children
            parent.children.borrow_mut().push(child.clone());
            // Update the new child's parent to be the parent node
            *child.parent.borrow_mut() = Rc::downgrade(parent);
        }

    }
}


#[cfg(test)]
mod test {
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::rc::Weak;

    use super::tree::*;
    #[test]
    fn t1() {
        let tree = Tree::new();
        {
            let child1 = Rc::new(Node {
                name: "dir1".to_string(), 
                size: 0, 
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(Vec::new()),
            }); 
            tree.append(&tree.root, &child1);  
            assert_eq!(tree.root.children.borrow().len(), 1);

            let child2 = Rc::new(Node {
                name: "file1".to_string(), 
                size: 20, 
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(Vec::new()),
            });
            tree.append(&child1, &child2); 
            assert_eq!(child1.children.borrow().len(), 1);

            tree.append(&child1, &Rc::new(Node {
                name:"file2".to_string(), 
                size:25,
                parent: RefCell::new(Weak::new()),
                children:RefCell::new(vec![])
            })); 
            assert_eq!(child1.children.borrow().len(), 2);

        }
        tree.root.walk();

        let my_node : Rc<Node> = tree.root.get_child2("dir1".to_string());
        assert_eq!(my_node.name, "dir1".to_string());
        
    }
}