pub fn get_single_line_comment(ext: &str) -> &str {
    match ext {
        "rs" | "zig" | "c" | "cpp" | "java" | "js" | "ts" | "go" | "fs" | "fsx" | "cs" | "h" | "hpp" => "//",
        "py" => "#",
        _ => panic!(".{} is not yet implemented", ext)
    }
}

pub fn get_open_multi_line_comment(ext: &str) -> &str {
    match ext {
        "rs" | "c" | "cpp" | "java" | "js" | "ts" | "go" | "fs" | "fsx" | "cs" | "h" | "hpp" => "/*",
        _ => panic!(".{} is not yet implemented", ext)
    }
}

pub fn get_closed_multi_line_comment(ext: &str) -> &str {
    match ext {
        "rs" | "c" | "cpp" | "java" | "js" | "ts" | "go" | "fs" | "fsx" | "cs" | "h" | "hpp" => "*/",
        _ => panic!(".{} is not yet implemented", ext)
    }
}
 
pub fn find_dot(src: &str) -> usize {
    let mut i = 0usize;
    let mut idx = 0usize;
    for ch in src.chars() {
        if ch == '.' {
            idx = i;
        }
        i += 1;
    }
    idx
}

pub fn get_extension(src: &str) -> &str {
    &src[find_dot(src)+1..src.len()]
}

pub fn write_to(src: &mut String, to_write: &str) {
    src.push_str(to_write);
    src.push('\n');
}

pub fn process_line(l: &str, com: &mut String, nl: &mut String, max: usize, mut in_comment: bool, mut in_string: bool, single_key: &str, open_key: &str, closed_key: &str) -> (bool, bool) {
    let line = l.trim();
    if !line.is_empty() && line.len() >= max {
        // in_string

        if !in_comment {
            if !in_string {
                if line.contains(single_key) && !l.contains(open_key) {
                    write_to(nl, &l[0..l.find(single_key).unwrap()]);
                    write_to(com, &l[l.find(single_key).unwrap()..l.len()]);
                    return (in_comment, in_string);
                }
                else if l.contains(single_key) && l.contains(open_key) {
                    if l.find(open_key).unwrap() < l.find(single_key).unwrap() {
                        in_comment = true;
                    }
                    else {
                        write_to(nl, &l[0..l.find(single_key).unwrap()]);
                        write_to(com, &l[l.find(single_key).unwrap()..l.len()]);
                    }
                }
                else if !l.contains(single_key) && l.contains(open_key) {
                    in_comment = true;
                    write_to(nl, &l[0..l.find(open_key).unwrap()]);
                    write_to(com, &l[l.find(open_key).unwrap()..l.len()]);
                }
                else if l.contains(open_key) && l.contains(closed_key) {
                    write_to(nl, &l[0..l.find(open_key).unwrap()]);
                    write_to(com, &l[l.find(open_key).unwrap()..l.find(closed_key).unwrap()+closed_key.len()]);
                    write_to(nl, &l[l.find(closed_key).unwrap()+closed_key.len()..l.len()]);
                }
                else {
                    nl.push_str(&l);
                }
            }
        }
        else {
            if !in_string {
                if l.contains(closed_key) && !l.contains(single_key) {
                    write_to(com, &l[0..l.find(closed_key).unwrap()+closed_key.len()]);
                    let tup = process_line(&l[l.find(closed_key).unwrap()+closed_key.len()..l.len()], com, nl, max, false, in_string, single_key, open_key, closed_key);
                    in_comment = tup.0;
                    in_string = tup.1;
                }
                else if l.contains(closed_key) && l.contains(single_key) {
                    if l.find(closed_key).unwrap() < l.find(single_key).unwrap() {
                        in_comment = false;
                        write_to(com, &l[0..l.find(closed_key).unwrap()+closed_key.len()]);
                    }
                }
                else {
                    write_to(com, &l);
                }
            }
        }
    }
    else if !line.is_empty() {
        if !in_comment {
            write_to(nl, &l);
        }
        else {
            write_to(com, &l);
        }
    }
    (in_comment, in_string)
}