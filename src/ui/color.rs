use pancurses::*;

pub const COLOR_DEFAULT: i16 = COLOR_WHITE + 1;

pub fn init() {
    start_color();
    use_default_colors();
    for fg in COLOR_BLACK..=COLOR_DEFAULT {
        for bg in COLOR_BLACK..=COLOR_DEFAULT {
            let fg_color = if fg == COLOR_DEFAULT { -1 } else { fg };
            let bg_color = if bg == COLOR_DEFAULT { -1 } else { bg };
            init_pair(index(fg, bg), fg_color, bg_color);
        }
    }
}

pub fn pair(fg: i16, bg: i16) -> chtype {
    COLOR_PAIR(index(fg, bg) as chtype)
}

pub fn fg(fg: i16) -> chtype {
    pair(fg, COLOR_DEFAULT)
}

fn index(fg: i16, bg: i16) -> i16 {
    // each color code is 4 bits and the resulting index can't be 0
    (fg << 4 | bg) + 1
}
