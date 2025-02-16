use ratatui::{prelude::*, widgets::*};
use crate::states::App;


pub fn render(f: &mut Frame, app: &mut App, area: Rect) {
    let score = Line::from(vec![
        " Score: ".into(),
        app.score.to_string().yellow().bold(),
    ]);

    let score_header = Paragraph::new(Line::from(score))
        .centered()
        .block(Block::default() );
    
    f.render_widget(score_header, area);
}