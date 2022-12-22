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
    
    for l in file.split("\n") {
        let tup = process_line(l, extension);
        if !tup.0.is_empty() || (tup.0.is_empty() && tup.1.is_empty())  {
            write_to(&mut new_file_content, &tup.0);
        }
        if !tup.1.is_empty() {
            write_to(&mut com_file_content, &tup.1);
        }
    }

    if !new_file_content.is_empty() {
        std::fs::write(path, &new_file_content[0..new_file_content.len()-1]).expect("Failed");
    }
    if !com_file_content.is_empty() {
        std::fs::write(path.to_owned() + ".comm", &com_file_content[0..com_file_content.len()-1]).expect("Failed");
    }

}