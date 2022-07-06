use int_enum::IntEnum;
use enum_iterator::{all, last, cardinality, Sequence};
use std::fmt;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Sequence, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value() as usize
}

pub fn value_to_color_string(value: usize) -> String {
    // This assertation checks whether provided enum is "correct".
    // Specifically it checks if enum size is equal to the last element "idx"
    // Yes, it can miss some cases. E.g. enum { A = 0, B = 100, C = 2 }
    // assert_eq!(cardinality::<ResistorColor>() - 1, color_to_value(last::<ResistorColor>().unwrap()));

    // match colors().get(value) {
    //     Some(color) => color.to_string(),
    //     None => String::from("value out of range"),
    // }

    // Alternative solution
    match ResistorColor::from_int(value as u8) {
        Ok(val) => format!("{:?}", val),
        Err(_) => "value out of range".to_string()
    }
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}
