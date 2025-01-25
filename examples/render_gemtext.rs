use argo::{GemFile, Line};
use crossterm::event::{self, Event};
use ratatui::Frame;

// How many lines we expect to fit the screen.
// This will allocate a buffer of this size to fit visible lines.
// Underestimating this will limit the amount of lines drawn at any time,
// while overestimating will allocate extra memory.
const SCREEN_SIZE_LINES: usize = 100;
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
    let text =         "Hello World!
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
";
    let lines: Vec<&str> = text.lines().collect();
    let lines = lines.as_slice();
    let buf: &mut [Line] = &mut [Line::EOF; SCREEN_SIZE_LINES];

    let gemfile = GemFile::new_from_lines(buf, &lines, SCREEN_SIZE_LINES);
    frame.render_stateful_widget(gemfile, frame.area(), &mut true);
}
