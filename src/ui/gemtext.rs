use crate::{GemFile, Line};
use ratatui::{
    style::Stylize,
    text::Line as TuiLine,
    widgets::{Paragraph, StatefulWidget, Widget, Wrap},
};
use std::{format, string::String, vec::Vec};

impl StatefulWidget for GemFile<'_> {
    type State = bool;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _: &mut Self::State,
    ) {
        let text: Vec<TuiLine> = self.lines.iter().map(|line| (*line).into()).collect();

        Paragraph::new(text)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}

impl<'a> From<Line<'a>> for TuiLine<'a> {
    fn from(val: Line<'a>) -> Self {
        match val {
            Line::Text(text) => TuiLine::from(text),
            Line::H1(text) => TuiLine::from(text.bold().underlined().italic()),
            Line::H2(text) => TuiLine::from(text.bold().italic()),
            Line::H3(text) => TuiLine::from(text.italic()),
            Line::Link(link, opt_text) => TuiLine::from(format!(
                "{}{}",
                if let Some(text) = opt_text {
                    text
                } else {
                    link
                },
                if opt_text.is_some() {
                    format!("({link})")
                } else {
                    String::new()
                }
            )),
            Line::ListItem(text) => TuiLine::from(format!("•{text}")),
            Line::Quote(text) => TuiLine::from(format!(">{text}")).italic(),
            Line::ModeToggle | Line::EOF => TuiLine::from(""),
        }
    }
}
