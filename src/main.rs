
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(x) = line {

                println!("data: {x}");
            }
        }
    } else {
        println!("Unable to open file");
    }
}


pub struct StyledTree {
    pub root: RefCell<Rc<StyledNode>>,
}
pub struct StyledNode {
//    pub node: Node,
    pub children: RefCell<Vec<Rc<StyledNode>>>,
    parent: RefCell<Weak<StyledNode>>
    
}

impl StyledTree {
    pub fn new() -> Self {
        StyledTree {
            root: RefCell::new(Rc::new(StyledNode {
//                node: Node { node_type: NodeType::Comment(String::from("comment")), children: vec![] },
                children: RefCell::new(vec![]),
                parent: RefCell::new(Default::default()),
                //specified_values: Default::default()
            }))
        }
    }
    pub fn make(&self) -> Rc<StyledNode> {
        Rc::new(StyledNode{
/*             node: Node {
                node_type: NodeType::Comment(String::from("comment")),
                Children: vec![] 
            }, */
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
            //specified_values: Default::default()
        })
    }
    pub fn make_with(&self, 
              children:RefCell<Vec<Rc<StyledNode>>>) -> Rc<StyledNode> {
        let rc =  Rc::new(StyledNode {
            children,
            parent: RefCell::new(Default::default()),
        });
        for ch in rc.children.borrow().iter() {
            *ch.parent.borrow_mut() = Rc::downgrade(&rc);
        }
        return rc;
    }
    pub fn set_root(&self, node:&Rc<StyledNode>) {
        *self.root.borrow_mut() = Rc::clone(node);
    }
    pub fn append(&self, parent:&Rc<StyledNode>, child:&Rc<StyledNode>) {
        parent.children.borrow_mut().push(Rc::clone(child));
        *child.parent.borrow_mut() = Rc::downgrade(parent);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let tree = StyledTree::new();
        let root = tree.make();
        let child1 = tree.make();
        let child2 = tree.make();
        tree.set_root(&root);
        tree.append(&root,&child1);
        tree.append(&root,&child2);
        assert_eq!(5, 5);
    }
}
