use std::{error::Error, io};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

mod states;
mod components;


fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = states::App::new();
    run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: states::App) -> io::Result<()> {
    // main loop - drawing and handling events
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                use KeyCode::*;
                match key.code {
                    Char('q') | Esc => return Ok(()),
                    _ => {}
                }
            }
        }
    }
}

// main drawing function
pub fn ui(f: &mut Frame, app: &mut states::App) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10), // Score section
            Constraint::Percentage(80), // Game board
            Constraint::Percentage(10), // Controls section
        ])
        .split(f.size());

    let board_area = outer_layout[1];

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(25), // Left padding
            Constraint::Percentage(50), // Board
            Constraint::Percentage(25), // Right padding
        ])
        .split(board_area);

    let score_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(25), // Left padding
            Constraint::Percentage(50), // Centered Score
            Constraint::Percentage(25), // Right padding
        ])
        .split(outer_layout[0]);

    let controls_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(25), // Left padding
            Constraint::Percentage(50), // Centered Controls
            Constraint::Percentage(25), // Right padding
        ])
        .split(outer_layout[2]);

    // header - score
    components::header::render(f, app, score_layout[1]);   
    // board
    components::board::render(f, app, inner_layout[1]); 
    // footer
    components::footer::render(f, app, controls_layout[1]);
}