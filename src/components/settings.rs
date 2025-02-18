use ratatui::{prelude::*, widgets::*};
use crate::states::App;
use crate::helpers::centered_rect;

pub fn render(f: &mut Frame, app: &mut App, _area: Rect) {
    let popup_block = Block::default()
        .title("SETTINGS:")
        .borders(Borders::ALL).border_type(BorderType::Double).border_style(Color::White)
        .style(Style::default().bg(Color::DarkGray));

    let area = centered_rect(50, 25, f.area());

    f.render_widget(popup_block, area);

    let popup_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);

    let header_text = Paragraph::new("Select Difficulty (grid size):")
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);

    f.render_widget(header_text, popup_chunks[0]);

    let current_difficulty = app.board.size;

    let difficulty = Text::from(vec![
        Line::from(vec![
            " Easy ".into(),
            "(4x4)".yellow().bold(),
            if current_difficulty == 4 { " <- Current".red().bold() } else { "".into() }
        ]),
        Line::from(vec![
            " Medium ".into(),
            "(5x5)".yellow().bold(),
            if current_difficulty == 5 { " <- Current".red().bold() } else { "".into() }
        ]),
        Line::from(vec![
            " Hard ".into(),
            "(6x6)".yellow().bold(),
            if current_difficulty == 6 { " <- Current".red().bold() } else { "".into() }
        ])
    ]);

    let difficulty_selection = Paragraph::new(difficulty)
        .centered()
        .wrap(Wrap { trim: true })
        .block(Block::default());

    f.render_widget(difficulty_selection, popup_chunks[1]);


    let change_difficulty_text = Paragraph::new(Line::from(vec![
        " Move Up ".into(), "<+>".yellow().bold(),
        " Move Down ".into(), "<->".yellow().bold(),
        " Close ".into(), "<ESC>".yellow().bold(),
    ])).centered();

    f.render_widget(change_difficulty_text, popup_chunks[3]);
}
