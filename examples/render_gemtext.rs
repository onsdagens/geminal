use argo::GemFile;
use crossterm::event::{self, Event};
use ratatui::{text::Text, widgets::Paragraph, Frame};

fn main() {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame) {
    let gemfile = GemFile::parse(
        "Hello World!
Here is a line
here is a super long line i wonder what will happen when the length of the line overruns the width of the terminal, i would really love to see it wrap automagically
# Here is a header
## Here is a h2
### Here is a h3
> This is a quote
=> hereislink
=> hereislink with optional text
*here is a list item
*here is another list item
"
        .lines(),
    );
    frame.render_stateful_widget(gemfile, frame.area(), &mut true);
}
