/// Represents a 3-Byte RGB color value.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    /// Return the corresponding Serato DJ Pro Hotcue color for this color.
    ///
    /// Hotcue colors stored in Serato's metadata are not necessarily the same as displayed in the
    /// UI. Serato DJ Intro/Pro/Lite always stores colors as shown in the legacy [Serato DJ
    /// Intro](https://serato.com/dj/intro).
    ///
    /// Serato DJ Pro and Lite then map the stored color to their own color palette. If the color
    /// is not in the palette, this returns the color unchanged.
    ///
    /// ```
    /// use triseratops::tag::color::Color;
    ///
    ///
    /// // This color is in the Serato DJ Intro palette...
    /// let intro_color = Color { red: 0xCC, green: 0x88, blue: 0x00 };
    /// let pro_color = intro_color.into_pro_hotcue_color();
    ///
    /// assert_eq!(pro_color, Color { red: 0xF8, green: 0x82, blue: 0x1A });
    ///
    /// // ... and this color isn't.
    /// let non_intro_color = Color { red: 0xC0, green: 0xFF, blue: 0xEE };
    /// let non_pro_color = non_intro_color.into_pro_hotcue_color();
    ///
    /// assert_eq!(non_pro_color, Color { red: 0xC0, green: 0xFF, blue: 0xEE });
    /// ```
    pub fn into_pro_hotcue_color(self) -> Self {
        if let Some(index) = HOTCUE_COLORS_INTRO.iter().position(|&x| x == self) {
            return HOTCUE_COLORS_PRO[index];
        }

        self
    }

    /// Return the corresponding Serato DJ Intro Hotcue (i.e. Metadata) color for this color.
    ///
    /// Hotcue colors stored in Serato's metadata are not necessarily the same as displayed in the
    /// UI. Serato DJ Intro/Pro/Lite always stores colors as shown in the legacy [Serato DJ
    /// Intro](https://serato.com/dj/intro).
    ///
    /// Serato DJ Pro and Lite then map the stored color to their own color palette. If the color
    /// is not in the palette, this returns the color unchanged.
    ///
    /// ```
    /// use triseratops::tag::color::Color;
    ///
    ///
    /// // This color is in the Serato DJ Pro palette...
    /// let pro_color = Color { red: 0xF8, green: 0x82, blue: 0x1A };
    /// let intro_color = pro_color.into_intro_hotcue_color();
    ///
    /// assert_eq!(intro_color, Color { red: 0xCC, green: 0x88, blue: 0x00 });
    ///
    /// // ... and this color isn't.
    /// let non_pro_color = Color { red: 0xC0, green: 0xFF, blue: 0xEE };
    /// let non_intro_color = non_pro_color.into_intro_hotcue_color();
    ///
    /// assert_eq!(non_intro_color, Color { red: 0xC0, green: 0xFF, blue: 0xEE });
    /// ```
    pub fn into_intro_hotcue_color(self) -> Self {
        if let Some(index) = HOTCUE_COLORS_PRO.iter().position(|&x| x == self) {
            return HOTCUE_COLORS_INTRO[index];
        }

        self
    }
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

const HOTCUE_COLORS_INTRO: [Color; 18] = [
    Color {
        red: 0xCC,
        green: 0x00,
        blue: 0x00,
    },
    Color {
        red: 0xCC,
        green: 0x44,
        blue: 0x00,
    },
    Color {
        red: 0xCC,
        green: 0x88,
        blue: 0x00,
    },
    Color {
        red: 0xCC,
        green: 0xCC,
        blue: 0x00,
    },
    Color {
        red: 0x88,
        green: 0xCC,
        blue: 0x00,
    },
    Color {
        red: 0x44,
        green: 0xCC,
        blue: 0x00,
    },
    Color {
        red: 0x00,
        green: 0xCC,
        blue: 0x00,
    },
    Color {
        red: 0x00,
        green: 0xCC,
        blue: 0x44,
    },
    Color {
        red: 0x00,
        green: 0xCC,
        blue: 0x88,
    },
    Color {
        red: 0x00,
        green: 0xCC,
        blue: 0xCC,
    },
    Color {
        red: 0x00,
        green: 0x88,
        blue: 0xCC,
    },
    Color {
        red: 0x00,
        green: 0x44,
        blue: 0xCC,
    },
    Color {
        red: 0x00,
        green: 0x00,
        blue: 0xCC,
    },
    Color {
        red: 0x44,
        green: 0x00,
        blue: 0xCC,
    },
    Color {
        red: 0x88,
        green: 0x00,
        blue: 0xCC,
    },
    Color {
        red: 0xCC,
        green: 0x00,
        blue: 0xCC,
    },
    Color {
        red: 0xCC,
        green: 0x00,
        blue: 0x88,
    },
    Color {
        red: 0xCC,
        green: 0x00,
        blue: 0x44,
    },
];

const HOTCUE_COLORS_PRO: [Color; 18] = [
    Color {
        red: 0xC0,
        green: 0x26,
        blue: 0x26,
    },
    Color {
        red: 0xDB,
        green: 0x4E,
        blue: 0x27,
    },
    Color {
        red: 0xF8,
        green: 0x82,
        blue: 0x1A,
    },
    Color {
        red: 0xFA,
        green: 0xC3,
        blue: 0x13,
    },
    Color {
        red: 0x4E,
        green: 0xB6,
        blue: 0x48,
    },
    Color {
        red: 0x00,
        green: 0x68,
        blue: 0x38,
    },
    Color {
        red: 0x1F,
        green: 0xAD,
        blue: 0x26,
    },
    Color {
        red: 0x8D,
        green: 0xC6,
        blue: 0x3F,
    },
    Color {
        red: 0x2B,
        green: 0x36,
        blue: 0x73,
    },
    Color {
        red: 0x1D,
        green: 0xBE,
        blue: 0xBD,
    },
    Color {
        red: 0x0F,
        green: 0x88,
        blue: 0xCA,
    },
    Color {
        red: 0x16,
        green: 0x30,
        blue: 0x8B,
    },
    Color {
        red: 0x17,
        green: 0x3B,
        blue: 0xA2,
    },
    Color {
        red: 0x5C,
        green: 0x3F,
        blue: 0x97,
    },
    Color {
        red: 0x68,
        green: 0x23,
        blue: 0xB6,
    },
    Color {
        red: 0xCE,
        green: 0x35,
        blue: 0x9E,
    },
    Color {
        red: 0xDC,
        green: 0x1D,
        blue: 0x49,
    },
    Color {
        red: 0xC7,
        green: 0x11,
        blue: 0x36,
    },
];
