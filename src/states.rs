use crate::helpers::{random_tile, random_two_or_four};

pub struct Board {
	pub size: u32,
	pub board: Vec<Vec<u32>>
}

#[derive(PartialEq)]
pub enum Page {
	Settings,
	GameOver,
	Quit,
	Game,
}

#[derive(PartialEq)]

pub enum SettingsDir {
	Up,
	Down
}

#[derive(PartialEq)]

pub enum MoveDir {
	Up,
	Down,
	Left,
	Right,
}

pub struct App {
	// ui states
	pub page: Page,

	// game states
	pub board: Board,
	pub score: u64,
}

impl App {
	pub fn new(board_size: Option<usize>) -> App {
		let size = board_size.unwrap_or(4);

		let mut a = App {
			page: Page::Game,
			board: Board {
				size: size as u32,
				board: vec![vec![0; size]; size],
			},
			score: 0,
		};

		// add two random tiles to the board
		let first_tile = random_tile(size as u32);
		let second_tile = random_tile(size as u32);
		a.board.board[first_tile / size][first_tile % size] = random_two_or_four();
		a.board.board[second_tile / size][second_tile % size] = random_two_or_four();

		a
	}

	pub fn change_difficulty(&mut self, direction: SettingsDir) {
		// Create a new App instance
		*self = App::new(
			if direction == SettingsDir::Up {
				Some((self.board.size + 2).min(8) as usize)
			} else {
				Some((self.board.size - 2).max(4) as usize)
			},
		);
		self.page = Page::Settings;
	}

	pub fn move_tiles(&mut self, direction: MoveDir) {
		// check if game is still played
		// make move (merge tiles, add new tiles, update score)
		// check if game is over -> show popup (and save max score to file)
	}
}