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

pub fn process_line(l: &str, com: &mut String, nl: &mut String, max: usize, mut in_comment: bool, mut in_string: bool, single_key: &str, open_key: &str, closed_key: &str) -> (bool, bool) {
    let line = l.trim();
    if !line.is_empty() && line.len() >= max {

        if l.contains('\"') {
            if l.find('\"').unwrap() != 0 {
                if &l[l.find('\"').unwrap()-1..=l.find('\"').unwrap()-1] != "\\" {
                    in_string = !in_string;
                }
            }
            else {
                in_string = !in_string;
            }
        }

        if !in_comment {
            if line.contains(single_key) && !l.contains(open_key) {
                nl.push_str(&l[0..l.find(single_key).unwrap()]);
                nl.push('\n');
                com.push_str(&l[l.find(single_key).unwrap()..l.len()]);
                com.push('\n');
            }
            else if l.contains(single_key) && l.contains(open_key) {
                if l.find(open_key).unwrap() < l.find(single_key).unwrap() {
                    in_comment = true;
                }
                else {
                    nl.push_str(&l[0..l.find(single_key).unwrap()]);
                    nl.push('\n');
                    com.push_str(&l[l.find(single_key).unwrap()..l.len()]);
                    com.push('\n');
                }
            }
            else if !l.contains(single_key) && l.contains(open_key) {
                in_comment = true;
                com.push_str(&l[l.find(open_key).unwrap()..l.len()]);
                com.push('\n');
            }
            else {
                nl.push_str(&l);
            }
        }
        else {
            if l.contains(closed_key) && !l.contains(single_key) {
                com.push_str(&l[0..l.find(closed_key).unwrap()+closed_key.len()]);
                com.push('\n');
                let tup = process_line(&l[l.find(closed_key).unwrap()+closed_key.len()..l.len()], com, nl, max, false, in_string, single_key, open_key, closed_key);
                in_comment = tup.0;
                in_string = tup.1;
                // in_comment = process_line(&l[l.find(closed_key).unwrap()+closed_key.len()..l.len()], com, nl, max, false, in_string, single_key, open_key, closed_key).0;
                // in_string = process_line(&l[l.find(closed_key).unwrap()+closed_key.len()..l.len()], com, nl, max, false, in_string, single_key, open_key, closed_key).1;
            }
            else if l.contains(closed_key) && l.contains(single_key) {
                if l.find(closed_key).unwrap() < l.find(single_key).unwrap() {
                    in_comment = false;
                    com.push_str(&l[0..l.find(closed_key).unwrap()+closed_key.len()]);
                    com.push('\n');
                }
            }
            else {
                com.push_str(&l);
                com.push('\n');
            }

        }


        // ...
    }
    else if !line.is_empty() {
        if !in_comment {
            nl.push_str(&l);
            nl.push('\n');
        }
        else {
            com.push_str(&l);
            com.push('\n');
        }
    }
    (in_comment, in_string)
}

// for i in 0..l.len()-max_len {

            //     if in_comment {
            //         com_file_content.push_str(&l[pos_open..l.len()]);
            //         pos_open = 0;
            //     }
            //     else {
            //         if !com_closed {
            //             com_file_content.push_str(&l[0..pos_closed]);
            //             com_closed = true;
            //         }
            //     }

            //     if &l[i..=i] == "\"" {
            //         if i != 0 {
            //             if &l[i-1..=i-1] != "\\" {
            //                 in_string = !in_string;
            //             }
            //         }
            //         else {
            //             in_string = !in_string;
            //         }
            //     }
                // if keys.contains(&&l[i..i+max_len]) && !in_string {
                //     if l[i..i+max_len].contains(&single_key) {
                //         com_file_content.push_str(&l[i..l.len()]);
                //         com_file_content.push('\n');
                //         break;
                //     }
                //     else if l[i..i+max_len].contains(&open_key) {
                //         in_comment = true;
                //         pos_open = i + l.find(open_key).unwrap();
                //     }
                //     else if l[i..i+max_len].contains(&closed_key) {
                //         in_comment = false;
                //         pos_closed = i + l.find(closed_key).unwrap();
                //         com_closed = false;
                //     }

                // }
                // else if in_string {
                //     if i == l.len() - single_key.len() - 1 {
                //         curr_file_line.push_str(&l[i..=i+single_key.len()]);
                //     }
                //     else {
                //         curr_file_line.push_str(&l[i..=i]);
                //     }
                // }
            // }