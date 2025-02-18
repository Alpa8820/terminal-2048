use crate::helpers::{random_tile, random_two_or_four};
use rand::Rng;

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
				Some((self.board.size + 1).min(6) as usize)
			} else {
				Some((self.board.size - 1).max(4) as usize)
			},
		);
		self.page = Page::Settings;
	}

	pub fn move_tiles(&mut self, direction: MoveDir) {
		// check if game is still played
		if self.page == Page::GameOver {
			return;
		}

		// 1. make move
		match direction {
			MoveDir::Up => {
				self.transpose_board();
				self.move_x_axis(&MoveDir::Left);
				self.transpose_board();
			},
			MoveDir::Down => {
				self.transpose_board();
				self.move_x_axis(&MoveDir::Right);
				self.transpose_board();
			},
			MoveDir::Left => self.move_x_axis(&MoveDir::Left),
			MoveDir::Right => self.move_x_axis(&MoveDir::Right)			
		};

		// 2. add new tile
		self.add_tile();

		// 3. update score
		self.update_score();

		// check if game is over -> show popup (and save max score to file)
		if self.check_game_over() {
			self.page = Page::GameOver;
		}
	}

	pub fn move_x_axis(&mut self, direction: &MoveDir) {
		for r in 0..self.board.size {
			let row: usize = r as usize;
	
			// slide and merge tiles in the specified direction
			self.slide(row, &direction);
	
			// merge tiles
			let mut merged = vec![false; self.board.size as usize]; // Track merged tiles
			for c in 0..self.board.size {
				let col: usize = c as usize;

				// merge tiles in column left (same row; col-1 => meaning move LEFT)
				if
					col > 0 &&
					self.board.board[row][col] == self.board.board[row][col - 1] &&
					*direction == MoveDir::Left &&
					!merged[col] && !merged[col - 1]
				{
					self.board.board[row][col - 1] *= 2;
					self.board.board[row][col] = 0;
					merged[col - 1] = true;
				}

				// merge tiles in column right (same row; col+1 => meaning move RIGHT)
				if
					(col as u32) < self.board.size - 1 &&
					self.board.board[row][col] == self.board.board[row][col + 1] &&
					*direction == MoveDir::Right &&
					!merged[col] && !merged[col + 1]
				{
					self.board.board[row][col + 1] *= 2;
					self.board.board[row][col] = 0;
					merged[col + 1] = true;
				}
			}
	
			// slide tiles again after merging
			self.slide(row, &direction);
		}
	}
	
	pub fn slide(&mut self, row_index: usize, direction: &MoveDir) {
		let mut new_row = Vec::new();
		let row = &self.board.board[row_index];
	
		// remove all zero tiles
		for tile in row {
			if *tile > 0 {
				new_row.push(*tile);
			}
		}
	
		// Add zero tiles to the end of new row
		while new_row.len() < self.board.size as usize {
			new_row.push(0);
		}
	
		// reverse the row for "right" movement
		if *direction == MoveDir::Right {
			new_row.reverse();
		}
	
		// save the new row
		self.board.board[row_index] = new_row;
	}	

	pub fn update_score(&mut self) {
		// if new highest score save to file
	}

	pub fn add_tile(&mut self) {
		// get all empty tiles
		let mut rng = rand::rng();
		let mut empty_tiles = Vec::new();
		for r in 0..self.board.size {
			for c in 0..self.board.size {
				if self.board.board[r as usize][c as usize] == 0 {
					empty_tiles.push((r as usize, c as usize));
				}
			}
		}

		// get random empty tile
		if empty_tiles.len() == 0 {
			return;
		} else if let Some(tile) = empty_tiles.get(rng.random_range(0..empty_tiles.len())) {
			// add a random tile (2 or 4) to that tile
			self.board.board[tile.0][tile.1] = random_two_or_four();
		}
	}

	pub fn check_game_over(&self) -> bool {
		// game over = board full, no legal moves
		let full = self.check_if_full();
		if !full {
			return false;
		}
		
		// check if any legal moves are possible
		for r in 0..self.board.size {
			for c in 0..self.board.size {
				let row = r as usize;
				let col = c as usize;

				// checking tiles in row above (row-1; same column)
				if row > 0 && self.board.board[row ][col] == self.board.board[row - 1][col] {
					return false;
				}

				// checking tiles in row below (row+1; same column)
				if (row as u32) < self.board.size - 1 && self.board.board[row][col] == self.board.board[row + 1][col] {
					return false;
				}

				// checking tiles in column left (same row; col-1)
				if col > 0 && self.board.board[row][col] == self.board.board[row][col - 1] {
					return false;
				}

				// checking tiles in column right (same row; col+1)
				if (col as u32) < self.board.size - 1 && self.board.board[row][col] == self.board.board[row][col + 1] {
					return false;
				}
			}
		}

		true
	}

	pub fn check_if_full(&self) -> bool {
		// no tile is 0
		for i in 0..self.board.size {
			for j in 0..self.board.size {
				if self.board.board[i as usize][j as usize] == 0 {
					return false;
				}
			}
		}
		true
	}

	pub fn transpose_board(&mut self) {
		let mut transposed = vec![vec![0; self.board.board.len()]; self.board.board[0].len()];
		for i in 0..self.board.board.len() {
			for j in 0..self.board.board[0].len() {
				transposed[j][i] = self.board.board[i][j];
			}
		}
		self.board.board = transposed;
	}
}