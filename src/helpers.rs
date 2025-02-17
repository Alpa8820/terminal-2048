use rand::Rng;
use ratatui::{layout::Rect, style::Color, layout::{Constraint, Direction, Layout}};

// generates a random number - either 2 or 4
pub fn random_two_or_four() -> u32 {
    let mut rng = rand::rng();
    if rng.random_bool(0.5) { 2 } else { 4 }
}

// generates a random tile index
pub fn random_tile(board_size: u32) -> u32 {
    let mut rng = rand::rng();
    rng.random_range(0..board_size * board_size)
}

// gets color of the tile based on tis value
pub fn get_tile_color(value: u32) -> Color {
    match value {
        2 => Color::Rgb(238, 228, 218),    // Light beige
        4 => Color::Rgb(237, 224, 200),    // Slightly darker beige
        8 => Color::Rgb(242, 177, 121),    // Light orange
        16 => Color::Rgb(245, 149, 99),    // Darker orange
        32 => Color::Rgb(246, 124, 95),    // Reddish-orange
        64 => Color::Rgb(246, 94, 59),     // Bright red-orange
        128 => Color::Rgb(237, 207, 114),  // Gold
        256 => Color::Rgb(237, 204, 97),   // Darker gold
        512 => Color::Rgb(237, 200, 80),   // Yellow-gold
        1024 => Color::Rgb(237, 197, 63),  // Deep yellow
        2048 => Color::Rgb(237, 194, 46),  // Golden yellow
        4096 => Color::Rgb(60, 58, 50),    // Dark gray
        8192 => Color::Rgb(60, 58, 50),    // Dark gray
        16384 => Color::Rgb(60, 58, 50),   // Dark gray
        32768 => Color::Rgb(60, 58, 50),   // Dark gray
        65536 => Color::Rgb(60, 58, 50),   // Dark gray
        _ => Color::Rgb(205, 193, 180),    // Default tile background
    }
}

// creates a centered rectangle - popup
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the center rectangle - popup
}
