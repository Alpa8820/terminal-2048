use ratatui::{prelude::*, widgets::*};
use crate::states::App;
use crate::helpers::centered_rect;

pub fn render(f: &mut Frame, app: &mut App, _area: Rect) {
    let popup_block = Block::default()
        .title("GAME OVER!")
        .borders(Borders::ALL).border_type(BorderType::Double).border_style(Color::White)
        .style(Style::default().bg(Color::Red));

    let area = centered_rect(50, 20, f.area());

    f.render_widget(popup_block, area);

    let popup_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)])
        .split(area);

    let game_over_text = Paragraph::new("You lost! Game Over!")
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);

    f.render_widget(game_over_text, popup_chunks[0]);

    let score = Line::from(vec![
        " Score: ".into(),
        app.score.to_string().yellow().bold(),
    ]);

    let score_paragraph = Paragraph::new(score)
        .centered()
        .block(Block::default());

    f.render_widget(score_paragraph, popup_chunks[1]);

    let instructions_text = Paragraph::new(Line::from(vec![
        " Quit ".into(), "<Q>".yellow().bold(),
        " New Game ".into(), "<N>".yellow().bold(),
        " Settings ".into(), "<S>".yellow().bold(),
    ])).centered();

    f.render_widget(instructions_text, popup_chunks[2]);
}
