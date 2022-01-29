//ComCol (aka Comment Collector) is a tool meant to remove and collect all comments from a certain file
//TODO: multiple comment keywords
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
        "rs" | "c" | "cpp" | "java" | "js" | "ts" | "go" | "fs" | "cs" => "//",
        "py" => "#",
        _ => panic!("Not implemented yet")
    }
}
fn get_multi_line_open_comment(ext: String) -> &'static str {
    match &ext as &str {
        "rs" | "c" | "cpp" | "java" | "js" | "ts" | "go" | "fs" | "cs" => "/*",
        "py" => "\'\'\'",
        _ => panic!("Not implemented yet")
    }
}
fn get_multi_line_close_comment(ext: String) -> &'static str {
    match &ext as &str {
        "rs" | "c" | "cpp" | "java" | "js" | "ts" | "go" | "fs" | "cs" => "*/",
        "py" => "\'\'\'",
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
    let multi_line_open_keyword = get_multi_line_open_comment(get_extension(path));
    let multi_line_close_keyword = get_multi_line_close_comment(get_extension(path));
    let mut new_content = String::new();
    let mut new_file = std::fs::File::create(&path).expect("Failed");
    let file_name = path.clone() + ".comm";
    let mut comments_file = std::fs::File::create(&file_name).expect("Failed");
    let mut comments_content = String::new();
    let mut opens = 0;

    for l in split {
        let line = String::from(l.trim());
        if line.starts_with(single_line_keyword) {
            comments.push(line.replace(single_line_keyword, ""));
        }
        else {
            if line.starts_with(multi_line_open_keyword) {
                opens += 1;
            }
            else {
                if line.starts_with(multi_line_close_keyword) {
                    opens -= 1;
                }
                else {
                    if opens != 0 {
                        comments.push(line.replace(multi_line_open_keyword, ""));
                    }
                    else {
                        if line.contains(&single_line_keyword) {
                            let mut ok = true;
                            let mut normal_line = String::from("");
                            let mut comm_line = String::from("");
                            let mut idx = 1;
                            for ch in line.chars() {
                                if ok {
                                    normal_line.push(ch);
                                }
                                else {
                                    comm_line.push(ch);
                                }
                                if single_line_keyword == "#" && ch == '#' {
                                    ok = false;
                                }
                                else if single_line_keyword == "//" && ch == '/' && line.chars().nth(idx).unwrap() == '/' {
                                    ok = false;
                                }
                                idx += 1;
                            }
                            new_content.push_str(&normal_line.replace('/', "") as &str);
                            new_content.push('\n');
                            comments.push(comm_line.replace('/', ""));
                            // println!("{normal_line}");
                            // println!("{comm_line}");
                        }
                        else {
                            new_content.push_str(l);
                            new_content.push('\n');
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", comments.clone());
    for com in comments {
        comments_content.push_str(&com as &str);
        comments_content.push('\n');
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