use pancurses::{initscr, endwin, Input, noecho};

fn main() {
    let window = initscr();
    window.printw("Hello, curses!");
    window.refresh();
    window.keypad(true);
    noecho();
    loop {
        match window.getch() {
            Some(Input::KeyF10) => break,
            Some(input) => {
                window.erase();
                window.printw(format!("{:?}\n", input));
            }
            None => ()
        }
    }
    endwin();
}
