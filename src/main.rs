
use std::borrow::BorrowMut;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
mod tree;
use crate::tree::tree::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;

#[derive(Debug)]
 enum Command {
    Cd (String),
    Ls,
    Dir (String),
    File (u32, String)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut command: Command;
    let tree: Tree = Tree::new();
    let mut node = tree.root.clone(); 
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(x) = line {
                println!("data: {x}");
                println!("node.name: {}", node.name);
                let command = parse_line(&x);
                //dbg!(command);
                match command{
                    Command::Cd (s) => { 
                    if s == "/".to_string() {
                        node = tree.root.clone();
                    } else if s == "..".to_string() {
                        node = node.get_parent();
                    } else { 
//                        tree.append(&node, &Rc::new(Node::new_dir(s.clone())));
                        node = node.get_child(s);
                    }},
                    Command::Ls => {},
                    Command::Dir (s) => { 
                        tree.append(&node, &Rc::new(Node::new_dir(s )));
                    }, 
                    Command::File (fs, s) => { 
                        tree.append(&node, &Rc::new(Node::new_file(s, fs)));
                    },  
                }
            }
        }
    } else {
        println!("Unable to open file");
    }
    tree.root.walk();

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(line: &String) -> Command{
    let tokens: Vec<&str> = line.split(" ").collect();
    if tokens[0].eq(&String::from("$")) {
        println!("line is a command");
        if tokens[1].eq(&String::from("ls")){
            println!("List command");
            return Command::Ls;
        }
        if tokens[1].eq(&String::from("cd")){
            println!("ChangeDir command {}", tokens[2].to_string());
            return Command::Cd(tokens[2].to_string());
        }
    }
    // Non-$ lines
    if tokens[0].eq(&String::from("dir")) {
        println!("directory listing{}", tokens[1].to_string());
        return Command::Dir(tokens[1].to_string());
    }
    if let Ok(f) = tokens[0].parse::<u32>() {
        println!("File: name: {}, size: {}", tokens[1].to_string(), f);
        return Command::File(f, tokens[1].to_string());
    }
//    assert!(false, "Unrecognised command: {0}", tokens[1]);
    return Command::Ls;
}



