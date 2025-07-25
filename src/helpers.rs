use rand::Rng;
use ratatui::{layout::Rect, style::Color, layout::{Constraint, Direction, Layout}};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::fs::File;


// generates a random number 2 or 4 as per the rules of the game (90% chance of 2, 10% chance of 4)
pub fn random_two_or_four() -> u32 {
    let mut rng = rand::rng();
    if rng.random_bool(0.9) { 2 } else { 4 }
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
        8192 => Color::Rgb(255, 127, 0),    // ff7f00
        16384 => Color::Rgb(255, 0, 222),   // ff00de
        32768 => Color::Rgb(12, 0, 255),   // 0c00ff
        65536 => Color::Rgb(0, 255, 102),   // 00ff66
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

// read highest score from file
pub fn get_highest_score() -> u64 {
    let mut file = File::open("highest_score.txt").unwrap();
    let mut score_str = String::new();
    file.read_to_string(&mut score_str).unwrap();
    score_str.trim().parse::<u64>().unwrap_or(0)
}

// save highest score to file
pub fn save_highest_score(score: u64) {
    //open file, clear content before writing new score
    if let Ok(mut file) = OpenOptions::new()
        .write(true)    
        .create(true)   
        .open("highest_score.txt")
    {
        let _ = writeln!(file, "{}", score);
    }
}