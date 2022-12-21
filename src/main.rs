// ComCol (aka Comment Collector) is a tool meant to remove and collect all comments from a certain file

mod utils;
use utils::*;

fn main() {
    
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let file = std::fs::read_to_string(&path).expect("Failed");
    let extension = get_extension(&path);
    let mut new_file_content = String::new();
    let mut com_file_content = String::new();
    
    println!("{}", extension);
    
    for l in file.split("\r\n") {
        let tup = process_line(l, extension);
        println!("{:?}", tup);
        write_to(&mut new_file_content, &tup.0.unwrap());
        write_to(&mut com_file_content, &tup.1.unwrap());
    }

    std::fs::write(path, new_file_content).expect("Failed");
    std::fs::write(path.to_owned() + ".comm", com_file_content).expect("Failed");

}