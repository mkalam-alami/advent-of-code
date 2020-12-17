extern crate pancurses;

use pancurses::{initscr, endwin, Input};

fn main() {
  let window = initscr();
  window.keypad(true);

  let mut exit = false;
  while !exit {
    window.refresh();

    let input = window.getch();
    window.clear();
    match input {
      Some(Input::KeyUp) => exit = true,
      Some(Input::Character(' ')) => exit = true,
      _ => ()
    }
  }

  window.delwin();
  endwin();
}
