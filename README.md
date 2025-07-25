# Terminal 2048 Game

A terminal-based implementation of the popular 2048 game, written in Rust! Play on different grid sizes, combine tiles with the same number, and aim for the highest score.

---

## 🎮 Features

- **Terminal-based UI** using the [`ratatui`](https://github.com/ratatui/ratatui) library
- **Intuitive keyboard controls** for smooth gameplay
- **Save & load highest score** to track your progress
- **Multiple difficulty levels:**
  - 🟢 **Easy** (4x4 grid)
  - 🔵 **Medium** (5x5 grid)
  - 🔴 **Hard** (6x6 grid)
- **Game states:**
  - 🕹️ **Playing**
  - 💀 **Game Over**
  - ⚙️ **Settings**

---

## 🖼 Gallery

![Game Screenshot](./assets/screenshot.png)

---

## 🎯 Controls

| Key | Action |
|-----|--------|
| **⬆️ ⬇️ ⬅️ ➡️** | Move tiles |
| **Q** | Quit the game |
| **N** | Start a new game |
| **S** | Open settings |
| **+** | Increase difficulty (bigger grid) |
| **-** | Decrease difficulty (smaller grid) |

---

## 🏗 Code Structure

📂 **Project Layout**

- [`src/main.rs`](src/main.rs) → Entry point of the application
- [`src/components`](src/components) → UI components (game board, game over screen, settings)
- [`src/helpers.rs`](src/helpers.rs) → Helper functions for game logic & UI
- [`src/states.rs`](src/states.rs) → Game state management

---

## 🛠 Installation & Usage

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/terminal-2048.git
   cd terminal-2048
   ```
2. Run the project using Cargo:
   ```sh
   cargo run
   ```


---

## 🙌 Acknowledgements

Special thanks to these amazing libraries:

- [`ratatui`](https://github.com/ratatui/ratatui) → Terminal UI framework
- [`crossterm`](https://github.com/crossterm-rs/crossterm) → Terminal input handling

---

🎲 **Enjoy playing Terminal 2048!** 🚀



Please star ⭐ this repo, if you like it.
