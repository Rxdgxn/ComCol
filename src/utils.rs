pub fn get_single_line_comment(ext: &str) -> &str {
    match ext {
        "rs" | "zig" | "c" | "cpp" | "java" | "js" | "ts" | "go" | "fs" | "fsx" | "cs" | "h" | "hpp" => "//",
        "py" => "#",
        _ => panic!(".{} is not yet implemented", ext)
    }
}

// pub fn get_open_multi_line_comment(ext: &str) -> &str {
//     match ext {
//         "rs" | "c" | "cpp" | "java" | "js" | "ts" | "go" | "fs" | "fsx" | "cs" | "h" | "hpp" => "/*",
//         _ => panic!(".{} is not yet implemented", ext)
//     }
// }

// pub fn get_closed_multi_line_comment(ext: &str) -> &str {
//     match ext {
//         "rs" | "c" | "cpp" | "java" | "js" | "ts" | "go" | "fs" | "fsx" | "cs" | "h" | "hpp" => "*/",
//         _ => panic!(".{} is not yet implemented", ext)
//     }
// }
 
pub fn find_last_dot(src: &str) -> usize {
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
    &src[find_last_dot(src)+1..src.len()]
}

pub fn write_to(src: &mut String, to_write: &str) {
    src.push_str(to_write);
    src.push('\n');
}

pub fn process_line(line: &str, ext: &str) -> (Option<String>, Option<String>) {
    let sgtok = get_single_line_comment(ext);
    let mut matched = true;
    let mut fq = 0usize;
    let tok_zones: Vec<usize> = line.match_indices(sgtok).map(|(i, _)|i).collect();
    let mut string_zones: Vec<(usize, usize)> = Vec::new();
    let mut found_sgtok = false;
    let mut sgtok_position = 0usize; // Default value

    for (i, ch) in line.chars().enumerate() {
        if ch == '\"' {
            if i > 0 {
                if line.chars().nth(i).unwrap() != '\\' {
                    matched = !matched;
                    match matched {
                        false => fq = i,
                        true => {
                            string_zones.push((fq, i));
                            fq = 0usize;
                        }
                    }
                }
            }
            else {
                matched = false;
            }
        }
    }

    for tzone in &tok_zones {
        for szones in &string_zones {
            if *tzone < szones.0 {
                found_sgtok = true;
                sgtok_position = *tzone;
                break;
            }
        }
    }
    if !found_sgtok {
        return (Some("".to_string()), Some(line.to_string()));
    }
    (Some(line[0..sgtok_position].to_string()), Some(line[sgtok_position+sgtok.len()..line.len()].to_string()))
}