use core::cmp::min;

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

// TODO: REMOVE THIS ANNOTATION ONCE THE FIELDS ARE IN USE (will be needed for move_window)
#[allow(dead_code)]
pub struct GemFile<'a> {
    draw_size: usize,
    current_line: usize,
    str_lines: &'a [&'a str],
    pub lines: &'a [Line<'a>],
}

impl<'a, 'b> GemFile<'a>
where
    'a: 'b,
{
    pub fn new_from_lines(
        buf: &'a mut [Line<'b>],
        lines: &'a [&str],
        draw_size: usize,
    ) -> GemFile<'a> {
        let read_len = min(draw_size, lines.len() - 1);
        for i in 0..read_len {
            buf[i] = lines[i].into();
        }
        GemFile {
            lines: buf,
            current_line: 0,
            str_lines: lines,
            draw_size,
        }
    }

    // this moves the currently rendered window according to specified delta
    pub fn move_window(&mut self, _delta: i32) {
        // the idea here is to only make the str -> Line conversion for
        // lines that were just now scrolled into view.
        todo!()
    }
}

pub enum ParseError {
    GetLineError,
}

// I think it's obvious what's going on here
// and clippy suggests unneccessary wordiness
#[allow(clippy::manual_strip)]
impl<'a> From<&'a str> for Line<'a> {
    fn from(l: &'a str) -> Self {
        if l.starts_with("=> ") {
            // trim special characters
            let l = l[2..].trim_start();
            if let Some((link, desc)) = l.split_once(' ') {
                Line::Link(link, Some(desc))
            } else if let Some((link, desc)) = l.split_once('\t') {
                Line::Link(link, Some(desc))
            } else {
                Line::Link(l, None)
            }
        } else if l.starts_with('#') {
            let l = &l[1..];
            if l.starts_with('#') {
                let l = &l[1..];
                if l.starts_with('#') {
                    Line::H3(&l[1..])
                } else {
                    Line::H2(l)
                }
            } else {
                Line::H1(l)
            }
        } else {
            Line::Text(l)
        }
    }
}
