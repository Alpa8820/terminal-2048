pub struct Board {
	pub size: u32,
	pub board: Vec<Vec<u32>>
}

pub enum Page {
	Menu,
	Game,
	GameOver,
	Quit,
}

pub struct App {
	// ui states
	pub board_size_input: u32,
	pub page: Page,

	// game states
	pub board: Board,
	pub score: u64,
}

pub struct Cell {
	pub value: u32,
	pub row: u32,
	pub col: u32,
}

impl App {
	pub fn new() -> App {
		App {
			board_size_input: 4,
			page: Page::Menu,
			board: Board {
				size: 4,
				board: vec![vec![0; 4]; 4],
			},
			score: 0,
		}
	}
}

impl Cell {

}


