use core::str::Lines;

#[derive(Clone, Copy, Debug)]
pub enum Line<'a> {
    // A line of standard text
    Text(&'a str),
    // Link (target, (optional) description) line
    Link(&'a str, Option<&'a str>),
    // Heading 1 line
    H1(&'a str),
    // Heading 2 line
    H2(&'a str),
    // Heading 3 line
    H3(&'a str),
    // List item line
    ListItem(&'a str),
    // Quote line
    Quote(&'a str),
    // Mode toggle line
    ModeToggle,
    EOF,
}

pub struct GemFile<'a> {
    pub lines: &'a [Line<'a>],
}

impl<'a> GemFile<'a> {
    pub fn parse<'b>(buf: &'a mut [Line<'b>], lines: &'a Lines<'b>) -> GemFile<'a> {
        for (i, l) in lines.clone().into_iter().enumerate() {
            if l.starts_with("=> ") {
                // trim special characters
                let l = l[2..].trim_start();
                if let Some((link, desc)) = l.split_once(" ") {
                    buf[i] = Line::Link(&link, Some(&desc));
                } else if let Some((link, desc)) = l.split_once("\t") {
                    buf[i] = Line::Link(&link, Some(&desc));
                } else {
                    buf[i] = Line::Link(&l, None);
                }
                //line_vec.push(Line::Link("Link".to_string(), Some("Link".to_string())));
            } else if l.starts_with("*") {
                buf[i] = Line::ListItem(&l[1..]);
            } else if l.starts_with("#") {
                let l = &l[1..];
                if l.starts_with("#") {
                    let l = &l[1..];
                    if l.starts_with("#") {
                        buf[i] = Line::H3(&l[1..]);
                    } else {
                        buf[i] = Line::H2(l)
                    }
                } else {
                    buf[i] = Line::H1(l);
                }
            } else {
                buf[i] = Line::Text(l);
            };
        }
        GemFile { lines: buf }
    }
}

pub enum ParseError {
    GetLineError,
}
