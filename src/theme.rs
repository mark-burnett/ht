/**
 *    Copyright 2018 Mark Burnett
 *
 *    Licensed under the Apache License, Version 2.0 (the "License");
 *    you may not use this file except in compliance with the License.
 *    You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing, software
 *    distributed under the License is distributed on an "AS IS" BASIS,
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *    See the License for the specific language governing permissions and
 *    limitations under the License.
 */
use ansi_term::{Color, Style};

#[derive(Debug)]
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

    pub static ref EMPTY: Theme = Theme {
        bool_value: Style::default(),
        header_name: Style::default(),
        header_value: Style::default(),
        key: Style::default(),
        null_value: Style::default(),
        number_value: Style::default(),
        status_error: Style::default(),
        status_info: Style::default(),
        status_message: Style::default(),
        status_success: Style::default(),
        string_value: Style::default(),

    };
}
