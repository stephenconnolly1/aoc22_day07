

pub mod tree {
//    use std::borrow::BorrowMut;
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
#[derive(PartialEq)]
    pub enum NodeType {
        File,
        Dir,
    }
    pub struct Node {
        pub name:       String,
        pub size:       RefCell<u32>,
        pub parent:     RefCell<Weak<Node>>,
        pub children:   RefCell<Vec<Rc<Node>>>,
        pub node_type:  NodeType,
    }
    impl Node {
        pub fn new() -> Node {
            Node {
                name: "".to_string(),
                size: RefCell::new(0),
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(Vec::new()),
                node_type: NodeType::Dir,
            }
        }
        pub fn new_file(name: String, size: u32) -> Node {
            let mut n = Node::new();
            n.name = name;
            n.size = RefCell::new(size);
            n.node_type = NodeType::File;
            return n;
        }
        pub fn new_dir(name: String) -> Node {
            let mut n = Node::new();
            n.name = name;
            return n;
        }
        pub fn walk(&self, level: &String) {
            let threshold: u32 = 100000;
            let level_loc = format!(" {}", level);
            if self.node_type==NodeType::File {
                // println!("{}filename: {}, size: {}", level_loc, self.name, self.size.borrow() );
            } 
            else {
                let s = *self.size.borrow();
//                println!("{}dirname: {}, size: {}", level_loc, self.name, s);
                if s <= threshold {
                    println!("Below threshold: {}", s);
                }
                for ch in self.children.borrow().iter().filter(|&x| x.node_type==NodeType::File) {
                    ch.walk(&level_loc);
                }
                for ch in self.children.borrow().iter().filter(|&x| x.node_type==NodeType::Dir) {
                    ch.walk(&level_loc);
                }
            }
//            println!("Sum of smaller directories: {}", sum_small_dirs);
        }
        pub fn get_child(&self, node_name: String) -> Rc<Node> {
            if let Some(r) =  self.children.borrow().iter().find(|&x|x.name == *node_name ) {
                return r.clone();
            }
            return Rc::new (Node::new());
        }
        pub fn get_parent(&self) -> Rc<Node> {
            assert!( self.name != "/".to_string());
            return self.parent.borrow().upgrade().unwrap();
        }
        pub fn update_total_sizes(&self) -> u32 {
            if self.node_type == NodeType::File {
                return *self.size.borrow();
            } 
            else { // Directory
                let mut s: u32 = 0;
                for ch in self.children.borrow().iter() {
                    s  = s + ch.update_total_sizes();
                }
                *self.size.borrow_mut() = s;
                println!("Dir:{}, total_size:{}, children:{}", self.name, s, self.children.borrow().len());
                return s;
            }
        }
    }

    impl Tree {
        pub fn new() -> Tree {
            let mut node = Node::new(); 
            node.name = "/".to_string();
            node.node_type = NodeType::Dir;
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
mod tests {
    use std::{rc::Rc, /*borrow::BorrowMut*/};

    use super::tree::*;
    #[test]
    fn t1() {
        let tree = Tree::new();
        {
            tree.append(&tree.root, &Rc::new(Node::new_file("file3".to_string(), 10001)));
            let dir1 =Rc::new(Node::new_dir("dir1".to_string())); 
            tree.append(&tree.root, &dir1);
            assert_eq!(tree.root.children.borrow().len(), 2);
            
//            let file1 = Rc::new(Node::new_file("file1".to_string(), 20));
            tree.append(&dir1, &Rc::new(Node::new_file("file1".to_string(), 20))); 
            assert_eq!(dir1.children.borrow().len(), 1);

            tree.append(&dir1, &Rc::new(Node::new_file("file2".to_string(), 25)));
            assert_eq!(dir1.children.borrow().len(), 2);

        }

        let my_node: Rc<Node> = tree.root.get_child("dir1".to_string());
        assert_eq!(my_node.name, "dir1".to_string());
        let my_node2: Rc<Node> = my_node.get_child("file2".to_string());
        assert_eq!(my_node2.name, "file2".to_string());
        assert_eq!(*my_node.size.borrow(), 0);
        assert_eq!(*my_node2.size.borrow(), 25);
        assert_eq!(my_node.update_total_sizes(), 45);
        assert_eq!(tree.root.update_total_sizes(), 10046);
        let mut cur_node = my_node2.clone();
        assert_eq!(cur_node.name, "file2".to_string());
        cur_node = cur_node.get_parent();
        assert_eq!(cur_node.name, "dir1".to_string());
        cur_node = cur_node.get_parent();
        assert_eq!(cur_node.name, "/".to_string());
        tree.root.update_total_sizes();
        tree.root.walk(&String::new());

        
    }
}