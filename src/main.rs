//ComCol (aka Comment Collector) is a tool meant to remove and collect all comments from a certain file

use std::io::prelude::*;
fn get_extension(f: &String) -> String {
    let mut t = String::from("");
    let mut ok = false;
    for ch in f.chars() {
        if ok {
            t.push(ch);
        }
        if ch == '.' {
            ok = true;
        }
    }
    return t
}
fn get_single_line_comment(ext: String) -> &'static str {
    match &ext as &str {
        "rs" | "c" | "cpp" | "java" | "js" | "ts" | "go" | "fs" | "cs" | "h" | "hpp" => "//",
        "py" => "#",
        _ => panic!("Not implemented yet")
    }
}


fn main() {
    
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    let path = args.first().unwrap();
    let f = std::fs::read_to_string(path).expect("Failed");
    let split = f.split("\n");

    let single_line_keyword = get_single_line_comment(get_extension(path));
    let mut new_content = String::new();
    let mut new_file = std::fs::File::create(&path).expect("Failed");
    let file_name = path.clone() + ".comm";
    let mut comments_file = std::fs::File::create(&file_name).expect("Failed");
    let mut comments_content = String::new();
    let mut opens = 0;

    for l in split {
        let mut end = false;
        let mut ok = true;
        let mut in_string = false;
        let mut comm_line = String::new();
        let mut new_line = String::new();
        if single_line_keyword == "//" {
            for (i, ch) in l.chars().enumerate() {
                if ch == '\'' || ch == '\"' {
                    if in_string {in_string = false;}
                    else {in_string = true;}
                }

                if ch == '/' {
                    if i+1 != l.len() && l.chars().nth(i+1).unwrap() == '/' {
                        ok = false;
                    }
                    else if i+1 != l.len() && l.chars().nth(i+1).unwrap() == '*' {
                        opens += 1;
                    }
                }
                else if ch == '*' && l.chars().nth(i+1).unwrap() == '/' {
                    opens -= 1;
                    if opens == 0 {
                        end = true;
                    }
                }

                if ok && opens == 0 && !end {
                    new_line.push(ch);
                }
                else {
                    comm_line.push(ch);
                }
            }
        }
        if !new_line.trim().is_empty() {
            new_content.push_str(&new_line);
            new_content.push('\n');
        }
        if !comm_line.trim().is_empty() {
            comments_content.push_str(&comm_line);
            comments_content.push('\n');
        }
    }

    match new_file.write_all(new_content.as_bytes()) {
        Err(e) => panic!("{}", e),
        Ok(_) => {}
    }
    match comments_file.write_all(comments_content.as_bytes()) {
        Err(e) => panic!("{}", e),
        Ok(_) => {}
    }
}