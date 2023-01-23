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

pub fn merge(v1: Vec<usize>, v2: Vec<usize>) -> Vec<usize> {
    let mut new = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < v1.len() && j < v2.len() {
        if v1[i] < v2[j] {
            new.push(v1[i]);
            i += 1;
        }
        else {
            new.push(v2[j]);
            j += 1;
        }
    }
    while i < v1.len() {
        new.push(v1[i]);
        i += 1;
    }
    while j < v2.len() {
        new.push(v2[j]);
        j += 1;
    }
    new
}

pub fn process_line(line: &str, ext: &str, matched: &mut bool, in_comment: &mut bool) -> (String, String) {
    let cltok = get_closed_multi_line_comment(ext);
    let optok = get_open_multi_line_comment(ext);
    if !*in_comment {
        let sgtok = get_single_line_comment(ext);
        let mut fq = 0usize;
        let tok_zones: Vec<usize> = line.match_indices(sgtok).map(|(i, _)|i).collect();
        let optok_zones: Vec<usize> = line.match_indices(optok).map(|(i, _)|i).collect();
        let tok_zones = merge(tok_zones, optok_zones);
        let mut string_zones: Vec<(usize, usize)> = Vec::new();
        let mut last_position = 0usize;

        for (i, ch) in line.chars().enumerate() {
            if ch == '\"' {
                if i > 0 {
                    if line.chars().nth(i).unwrap() != '\\' {
                        *matched = !*matched;
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
                    *matched = false;
                }
            }
        }

        for tok in &tok_zones {
            let mut ok = true;
            for szones in &string_zones {
                last_position = *tok;
                if *tok > szones.0 && *tok < szones.1 {
                    ok = false;
                    break;
                }
            }
            if ok && !string_zones.is_empty() {
                if last_position + std::cmp::max(optok.len(), sgtok.len()) >= line.len() {
                    if optok.len() > sgtok.len() || (optok.len() == sgtok.len() && &line[tok_zones[last_position]..tok_zones[last_position]+sgtok.len()] == sgtok) {
                        return (line[0..last_position].to_string(), line[last_position+sgtok.len()..line.len()].to_string());
                    }
                    else if optok.len() < sgtok.len() || (optok.len() == sgtok.len() && &line[tok_zones[last_position]..tok_zones[last_position]+sgtok.len()] == optok) {
                        *in_comment = true;
                        return (line[0..last_position].to_string(), line[last_position+sgtok.len()..line.len()].to_string());
                    }
                }
                else {
                    if &line[last_position..last_position+sgtok.len()] == sgtok {
                        return (line[0..last_position].to_string(), line[last_position+sgtok.len()..line.len()].to_string());
                    }
                    else if &line[last_position..last_position+optok.len()] == optok {
                        *in_comment = true;
                        return (line[0..last_position].to_string(), line[last_position+sgtok.len()..line.len()].to_string());
                    }
                }
            }
            else if ok && string_zones.is_empty() {
                if *tok + std::cmp::max(optok.len(), sgtok.len()) >= line.len() {
                    if optok.len() > sgtok.len() || (optok.len() == sgtok.len() && &line[tok_zones[*tok]..tok_zones[*tok]+sgtok.len()] == sgtok) {
                        return (line[0..*tok].to_string(), line[*tok+sgtok.len()..line.len()].to_string());
                    }
                    else if optok.len() < sgtok.len() || (optok.len() == sgtok.len() && &line[tok_zones[*tok]..tok_zones[*tok]+sgtok.len()] == optok) {
                        *in_comment = true;
                        return (line[0..*tok].to_string(), line[*tok+sgtok.len()..line.len()].to_string());
                    }
                }
                else {
                    if &line[*tok..(*tok)+sgtok.len()] == sgtok {
                        return (line[0..*tok].to_string(), line[*tok+sgtok.len()..line.len()].to_string());
                    }
                    else if &line[*tok..(*tok)+optok.len()] == optok {
                        *in_comment = true;
                        return (line[0..*tok].to_string(), line[*tok+sgtok.len()..line.len()].to_string());
                    }
                }
            }
        }
        return (line.to_string(), "".to_string());
    }
    if !line.contains(cltok) {
        ("".to_string(), line.to_string())
    }
    else {
        let cltok_pos = line.find(cltok).unwrap();
        *in_comment = false;
        // TODO: Continue to look at the rest of the line: ... /*___*/ ...
        (line[cltok_pos+cltok.len()..line.len()].to_string(), line[0..cltok_pos].to_string())
    }
}