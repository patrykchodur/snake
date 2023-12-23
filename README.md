# Rust Snake
This is a snake game implemented in rust. The main purpose of the game
is for me to learn rust and Terminal User Interface development, here
using [ratatui](https://ratatui.rs/) library.

## Usage
Just clone the project and run
```
cargo run
```

## TODOs
- Fix overlaping when snake goes out of the map
    - Wrap around map bounds
    - Move items when map resized
- Don't use the full terminal for playing
    - Add some UI info on the right
- Add option to restart the game
    - Take a look how tabs are done in ratatui tutorials for menu
- Add some options like map size or speed
- Change colors
- Add next move to array, so we can have a few in a row
- There was some bug when fruit was on the left edge of the screen,
  by which the fruit was displayed in the wrong pixel, the one to the right.
