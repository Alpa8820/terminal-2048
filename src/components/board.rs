use ratatui::{prelude::*, widgets::*};
use crate::states::App;

pub fn render(f: &mut Frame, app: &mut App, area: Rect) {
    let board = Paragraph::new(Line::from("board"))
        .centered()
        .block(Block::default().borders(Borders::ALL));
    
    f.render_widget(board, area);
}