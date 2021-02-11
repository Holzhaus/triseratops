/// Represents a 3-Byte RGB color value.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl From<u32> for Color {
    fn from(code: u32) -> Self {
        let blue = (code & 0xFF) as u8;
        let green = ((code >> 8) & 0xFF) as u8;
        let red = ((code >> 16) & 0xFF) as u8;
        Color { red, green, blue }
    }
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        (self.red as u32) << 16 | (self.green as u32) << 8 | self.blue as u32
    }
}
