// ComCol (aka Comment Collector) is a tool meant to remove and collect all comments from a certain file
// TODO: in_string

mod utils;
use utils::*;

fn main() {
    
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let file = std::fs::read_to_string(path).expect("Failed");
    let single_key = get_single_line_comment(get_extension(&path));
    let open_key = get_open_multi_line_comment(get_extension(&path));
    let closed_key = get_closed_multi_line_comment(get_extension(&path));
    let max_len = std::cmp::max(std::cmp::max(open_key.len(), closed_key.len()), single_key.len());
    let mut new_file_content = String::new();
    let mut com_file_content = String::new();
    let mut in_string = false;
    let mut in_comment = false;
    
    
    for l in file.split('\n') {
        let mut curr_file_line = String::new();
        let tup = process_line(l, &mut com_file_content, &mut curr_file_line, max_len, in_comment, in_string, single_key, open_key, closed_key);
        in_comment = tup.0;
        in_string = tup.1;
        if !curr_file_line.trim().is_empty() {
            new_file_content.push_str(&curr_file_line);
            new_file_content.push('\n');
        }
    }


    println!("{}", com_file_content);
    println!("--------------------");
    println!("{}", new_file_content);
}