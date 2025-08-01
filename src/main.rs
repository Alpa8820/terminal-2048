use std::{error::Error, io};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

mod states;
mod components;
pub mod helpers;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = states::App::new(None);
    let _ = run_app(&mut terminal, app);

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

        let board_size = app.board.size as usize;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                use KeyCode::*;
                match key.code {
                    Char('q') => return Ok(()),
                    Char('n') => app = states::App::new(Some(board_size)),
                    Char('s') => app.page = states::Page::Settings,
                    Esc => app.page = states::Page::Game,
                    Char('+') => app.change_difficulty(states::SettingsDir::Up),
                    Char('-') => app.change_difficulty(states::SettingsDir::Down),
                    Left => app.move_tiles(states::MoveDir::Left),
                    Right => app.move_tiles(states::MoveDir::Right),
                    Up => app.move_tiles(states::MoveDir::Up),
                    Down => app.move_tiles(states::MoveDir::Down),
                    _ => {}
                }
            }
        }
    }
}

// main drawing function
pub fn ui(f: &mut Frame, app: &mut states::App) {
    let area = f.area();

    let padding_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(5),
            Constraint::Percentage(90),
            Constraint::Percentage(5)
        ])
        .split(area);

    let outer = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(5),
            Constraint::Percentage(70),
            Constraint::Percentage(20),
            Constraint::Percentage(5)
        ])
        .split(padding_layout[1]);

    components::board::render(f, app, outer[1]);
    components::infos::render(f, app, outer[2]);

    // render popups if needed
    if app.page == states::Page::Settings {
        components::settings::render(f, app, area);
    } else if app.page == states::Page::GameOver {
        components::game_over::render(f, app, area);    
    }
}