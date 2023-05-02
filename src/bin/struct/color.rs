use std::fmt;

#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RGB ({}, {}, {}) {0:#X}{1:X}{2:X}",
            self.red, self.green, self.blue
        )
    }
}
