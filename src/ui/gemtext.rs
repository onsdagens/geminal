use crate::{GemFile, Line};
//use crossterm::style::Stylize;
use ratatui::{
    style::Stylize,
    text::{Line as TuiLine, Text},
    widgets::{Paragraph, StatefulWidget, Widget, Wrap},
    Frame,
};

impl StatefulWidget for GemFile {
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

impl Into<TuiLine<'static>> for Line {
    fn into(self) -> TuiLine<'static> {
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
                    link.clone()
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
        }
    }
}

impl Into<Text<'static>> for Line {
    fn into(self) -> Text<'static> {
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
                    link.clone()
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
        }
    }
}
