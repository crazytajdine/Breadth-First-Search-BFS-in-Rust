use std::{fs::read_to_string, io::Error};



fn read_file(path: &str) -> Result<String,Error> {
    
    read_to_string(path)
}

fn main() {
    read_file("./files/name.txt");
}
