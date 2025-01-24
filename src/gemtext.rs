use core::str::Lines;

#[derive(Clone, Debug)]
pub enum Line {
    // A line of standard text
    Text(String),
    // Link (target, (optional) description) line
    Link(String, Option<String>),
    // Heading 1 line
    H1(String),
    // Heading 2 line
    H2(String),
    // Heading 3 line
    H3(String),
    // List item line
    ListItem(String),
    // Quote line
    Quote(String),
    // Mode toggle line
    ModeToggle,
}

pub struct GemFile {
    pub lines: Vec<Line>,
}

impl GemFile {
    pub fn parse(lines: Lines) -> GemFile {
        let mut line_vec = vec![];
        for l in lines {
            if l.starts_with("=> ") {
                // trim special characters
                let l = l[2..].to_string();
                // trim leading whitespaces
                let l = l.trim_start();
                if let Some((link, desc)) = l.split_once(" ") {
                    line_vec.push(Line::Link(link.to_string(), Some(desc.to_string())));
                } else if let Some((link, desc)) = l.split_once("\t") {
                    line_vec.push(Line::Link(link.to_string(), Some(desc.to_string())));
                } else {
                    line_vec.push(Line::Link(l.to_string(), None));
                }
                //line_vec.push(Line::Link("Link".to_string(), Some("Link".to_string())));
            } else if l.starts_with("*") {
                line_vec.push(Line::ListItem(l[1..].to_string()));
            } else if l.starts_with("#") {
                let l = l[1..].to_string();
                if l.starts_with("#") {
                    let l = l[1..].to_string();
                    if l.starts_with("#") {
                        line_vec.push(Line::H3(l[1..].to_string()));
                    } else {
                        line_vec.push(Line::H2(l))
                    }
                } else {
                    line_vec.push(Line::H1(l));
                }
            } else {
                line_vec.push(Line::Text(l.to_string()))
            };
        }
        GemFile { lines: line_vec }
    }
}

pub enum ParseError {
    GetLineError,
}
