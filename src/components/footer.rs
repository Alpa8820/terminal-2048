use ratatui::{prelude::*, widgets::*};
use crate::states::App;

pub fn render(f: &mut Frame, app: &mut App, area: Rect) {
    let instructions = Line::from(vec![
        " Quit ".into(),
        "<Q> ".blue().bold(),
        " New Game ".into(),
        "<N>".blue().bold(),
        " Settings ".into(),
        "<S>".blue().bold(),
        " Move Up ".into(),
        "<↑>".blue().bold(),
        " Move Down ".into(),
        "<↓>".blue().bold(),
        " Move Left ".into(),
        "<←>".blue().bold(),
        " Move Right ".into(),
        "<→>".blue().bold(),
    ]);

    let info_footer = Paragraph::new(Line::from(instructions))
        .centered()
        .block(Block::default());
    
    f.render_widget(info_footer, area);
}