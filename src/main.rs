
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
mod tree;




fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(x) = line {
                println!("data: {x}");
                parse_line(&x);
            }
        }
    } else {
        println!("Unable to open file");
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(line: &String){
    let tokens: Vec<&str> = line.split(" ").collect();
    if tokens[0].eq(&String::from("$")) {
        println!("line is a command");
        if tokens[1].eq(&String::from("ls")){
            println!("List command");
        }
        else if tokens[1].eq(&String::from("cd")){
            println!("ChangeDir command");
        }
        else {
            assert!(false, "Unrecognised command: {0}", tokens[1]);
        }
    }

}
