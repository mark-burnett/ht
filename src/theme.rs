use ansi_term::{Color, Style};

pub struct Theme {
    pub status_info: Style,
    pub status_success: Style,
    pub status_message: Style,

    pub header_name: Style,
    pub header_value: Style,

    pub key: Style,

    pub bool_value: Style,
    pub null_value: Style,
    pub number_value: Style,
    pub status_error: Style,
    pub string_value: Style,
}

lazy_static! {
    pub static ref DEFAULT: Theme = Theme {
        bool_value: Color::Red.normal(),
        header_name: Color::Black.bold(),
        header_value: Color::Cyan.normal(),
        key: Color::Blue.normal(),
        null_value: Color::Black.bold(),
        number_value: Color::Purple.normal(),
        status_error: Color::Red.normal(),
        status_info: Color::Cyan.normal(),
        status_message: Color::Black.bold(),
        status_success: Color::Green.normal(),
        string_value: Color::Cyan.normal(),
    };
}
