use crate::{GemFile, Line};
use ratatui::{
    style::Stylize,
    text::{Line as TuiLine, Text},
    widgets::{Paragraph, StatefulWidget, Widget, Wrap},
};
use std::{format, string::ToString, vec::Vec};

impl<'a> StatefulWidget for GemFile<'a> {
    type State = bool;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _: &mut Self::State,
    ) {
        let text: Vec<TuiLine> = self.lines.iter().map(|line| line.clone().into()).collect();

        Paragraph::new(text)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}

impl<'a> Into<TuiLine<'a>> for Line<'a> {
    fn into(self) -> TuiLine<'a> {
        match self {
            Self::Text(text) => TuiLine::from(text),
            Self::H1(text) => TuiLine::from(text.bold().underlined().italic()),
            Self::H2(text) => TuiLine::from(text.bold().italic()),
            Self::H3(text) => TuiLine::from(text.italic()),
            Self::Link(link, opt_text) => TuiLine::from(format!(
                "{}{}",
                if let Some(text) = opt_text.clone() {
                    text
                } else {
                    link
                },
                if let Some(_) = opt_text {
                    format!("({})", link)
                } else {
                    "".to_string()
                }
            )),
            Self::ListItem(text) => TuiLine::from(format!("•{}", text)),
            Self::Quote(text) => TuiLine::from(format!(">{}", text)).italic(),
            Self::ModeToggle => TuiLine::from(""),
            Self::EOF => TuiLine::from(""),
        }
    }
}

impl<'a> Into<Text<'a>> for Line<'a> {
    fn into(self) -> Text<'a> {
        match self {
            Self::Text(text) => Text::from(text),
            Self::H1(text) => Text::from(text.bold().underlined().italic()),
            Self::H2(text) => Text::from(text.bold().italic()),
            Self::H3(text) => Text::from(text.italic()),
            Self::Link(link, opt_text) => Text::from(format!(
                "{}{}",
                if let Some(text) = opt_text.clone() {
                    text
                } else {
                    link
                },
                if let Some(_) = opt_text {
                    format!("({})", link)
                } else {
                    "".to_string()
                }
            )),
            Self::ListItem(text) => Text::from(format!("•{}", text)),
            Self::Quote(text) => Text::from(format!(">{}", text)).italic(),
            Self::ModeToggle => Text::from(""),
            Self::EOF => Text::from(""),
        }
    }
}
