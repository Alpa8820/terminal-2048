use ratatui::{prelude::*, widgets::*};
use std::rc::Rc;
use crate::states::App;
use crate::helpers::get_tile_color;


pub fn render(f: &mut Frame, app: &mut App, area: Rect) {
    let board_size = app.board.size;

    // generate layout for each cell (depending on board size)
    let row_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(25); board_size as usize])
        .split(area);

    let mut cells: Vec<Rc<[Rect]>> = Vec::with_capacity(board_size as usize);
    for row in row_areas.iter() {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(25); board_size as usize])
            .split(*row);
        cells.push(Rc::from(cols));
    }

    // fill and render this cells
    for x in 0..(board_size*board_size) as usize {
        let i = x / board_size as usize;
        let j = x % board_size as usize;

        let block_value = app.board.board[i][j];

        let mut block_text: String = " ".parse().unwrap();
        if block_value > 0 {
            block_text = block_value.to_string();
        }

        let game_blocks = Paragraph::new(block_text)
            .style(
                Style::default()
                    .fg(get_tile_color(block_value))
                    .add_modifier(Modifier::BOLD),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .padding(Padding::new(0, 1, cells[i][j].height / 3, 1)),
            )
            //center the text vertically and horizontally
            .alignment(Alignment::Center);

        f.render_widget(game_blocks, cells[i][j]);
    }
}