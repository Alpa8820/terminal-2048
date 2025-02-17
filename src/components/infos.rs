use ratatui::{prelude::*, widgets::*};
use crate::states::App;

pub fn render(f: &mut Frame, app: &mut App, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![
            Constraint::Percentage(15), // game title
            Constraint::Length(0),      // spacer
            Constraint::Length(1),      // score
            Constraint::Length(1),      // highest score
            Constraint::Min(0),         // flexible space
            Constraint::Percentage(20), // instructions
            Constraint::Length(1),      // spacer
            Constraint::Percentage(25), // movements
        ])
        .split(area);

    let game_header = Paragraph::new(Line::from("2048 Terminal Game".blue().bold()))
        .centered()
        .block(Block::default());

    let highest_score = Line::from(vec![
        " Highest Score: ".into(),
        app.score.to_string().yellow().bold(),
    ]);

    let score = Line::from(vec![
        " Score: ".into(),
        app.score.to_string().yellow().bold(),
    ]);

    let score_paragraph = Paragraph::new(score)
        .centered()
        .block(Block::default());

    let highest_score_paragraph = Paragraph::new(highest_score)
        .centered()
        .block(Block::default());

    let instructions_text = Text::from(vec![
        Line::from(vec![" Quit ".into(), "<Q>".green().bold()]),
        Line::from(vec![" New Game ".into(), "<N>".green().bold()]),
        Line::from(vec![" Settings ".into(), "<S>".green().bold()]),
    ]);

    let footer_instructions = Paragraph::new(instructions_text)
        .centered()
        .wrap(Wrap { trim: true })
        .block(Block::default());

    let movements_text = Text::from(vec![
        Line::from(vec![" Move Up ".into(), "<↑>".green().bold()]),
        Line::from(vec![" Move Down ".into(), "<↓>".green().bold()]),
        Line::from(vec![" Move Left ".into(), "<←>".green().bold()]),
        Line::from(vec![" Move Right ".into(), "<→>".green().bold()]),
    ]);

    let footer_movements = Paragraph::new(movements_text)
        .centered()
        .wrap(Wrap { trim: true })
        .block(Block::default());

    let block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded);

    f.render_widget(block, area);
    f.render_widget(game_header, layout[0]);
    f.render_widget(score_paragraph, layout[2]);   
    f.render_widget(highest_score_paragraph, layout[3]);    

    f.render_widget(footer_instructions, layout[5]);
    f.render_widget(footer_movements, layout[7]);
}
