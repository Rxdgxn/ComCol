//ComCol (aka Comment Collector) is a tool meant to remove and collect all comments from a certain file
//TODO: separate file for removed comments

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
        "rs" | "c" | "cpp" | "java" | "js" | "ts" | "go" => "//",
        "py" => "#",
        "lua" => "--",
        _ => panic!("Not implemented yet")
    }
}


fn main() {
    let mut comments: Vec<String> = vec![];
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    let path = args.first().unwrap();
    let f = std::fs::read_to_string(path).expect("Failed");
    let split = f.split("\n");

    let single_line_keyword = get_single_line_comment(get_extension(path));
    let mut new_content = String::new();

    let mut new_file = std::fs::File::create(&path).expect("Failed");

    for l in split {
        let line = String::from(l.trim());
        if line.starts_with(single_line_keyword) {
            comments.push(line.replace(single_line_keyword, ""));
        }
        else {
            new_content.push_str(l);
            new_content.push('\n');
        }
    }
    // println!("{:?}", comments);
    match new_file.write_all(new_content.as_bytes()) {
        Err(e) => panic!("{}", e),
        Ok(_) => {}
    }
}