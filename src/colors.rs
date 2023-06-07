use raylib::ffi::Color;

pub const DARK_GREY: Color = Color {
    r: 26,
    g: 31,
    b: 40,
    a: 255,
};

pub const GREEN: Color = Color {
    r: 47,
    g: 230,
    b: 23,
    a: 255,
};

pub const RED: Color = Color {
    r: 232,
    g: 18,
    b: 18,
    a: 255,
};

pub const ORANGE: Color = Color {
    r: 226,
    g: 116,
    b: 17,
    a: 255,
};

pub const YELLOW: Color = Color {
    r: 237,
    g: 234,
    b: 4,
    a: 255,
};

pub const PURPLE: Color = Color {
    r: 116,
    g: 0,
    b: 247,
    a: 255,
};

pub const CYAN: Color = Color {
    r: 21,
    g: 204,
    b: 209,
    a: 255,
};

pub const BLUE: Color = Color {
    r: 13,
    g: 64,
    b: 216,
    a: 255,
};

pub const LIGHT_BLUE: Color = Color {
    r: 59,
    g: 85,
    b: 162,
    a: 255,
};

pub const DARK_BLUE: Color = Color {
    r: 44,
    g: 44,
    b: 127,
    a: 255,
};

pub fn get_cell_colors() -> Vec<Color> {
    vec![DARK_GREY, GREEN, RED, ORANGE, YELLOW, PURPLE, CYAN, BLUE]
}
