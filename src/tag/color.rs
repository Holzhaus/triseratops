//! Hotcue and Track Colors Helpers
//!
//! # Hotcue Colors
//!
//! The on-screen representation of hotcue colors can differ slightly from what's stored in the
//! [`Serato Markers`](super::markers) and [`Serato Markers2`](super::markers2) tags depending on
//! whether Serato DJ Pro, Serato DJ Lite or Serato DJ Intro is used.
//!
//! Both Serato DJ Pro and Serato DJ Intro let the user choose from a palette of 18 different
//! colors.
//! In contrast to the latter which just displays the exact same colors that are saved in the
//! metadata, both Serato DJ Lite and Serato DJ Pro map the colors to another color palette:
//!
//! | Palette Index | Default Hotcue  | Serato DJ Intro/Metadata                                                                                        | Serato DJ Pro/Lite                                                                                            |
//! | ------------- | --------------- | --------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
//! |         **1** |               1 | ![CC0000](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/intro_CC0000.gif) `#CC0000` | ![C02626](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/pro_C02626.gif) `#C02626` |
//! |             2 |                 | ![CC4400](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/intro_CC4400.gif) `#CC4400` | ![DB4E27](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/pro_DB4E27.gif) `#DB4E27` |
//! |         **3** |               2 | ![CC8800](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/intro_CC8800.gif) `#CC8800` | ![F8821A](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/pro_F8821A.gif) `#F8821A` |
//! |         **4** |               4 | ![CCCC00](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/intro_CCCC00.gif) `#CCCC00` | ![FAC313](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/pro_FAC313.gif) `#FAC313` |
//! |             5 |                 | ![88CC00](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/intro_88CC00.gif) `#88CC00` | ![4EB648](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/pro_4EB648.gif) `#4EB648` |
//! |             6 |                 | ![44CC00](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/intro_44CC00.gif) `#44CC00` | ![006838](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/pro_006838.gif) `#006838` |
//! |             7 |               5 | ![00CC00](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/intro_00CC00.gif) `#00CC00` | ![1FAD26](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/pro_1FAD26.gif) `#1FAD26` |
//! |             8 |                 | ![00CC44](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/intro_00CC44.gif) `#00CC44` | ![8DC63F](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/pro_8DC63F.gif) `#8DC63F` |
//! |             9 |                 | ![00CC88](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/intro_00CC88.gif) `#00CC88` | ![2B3673](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/pro_2B3673.gif) `#2B3673` |
//! |            10 |               7 | ![00CCCC](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/intro_00CCCC.gif) `#00CCCC` | ![1DBEBD](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/pro_1DBEBD.gif) `#1DBEBD` |
//! |            11 |                 | ![0088CC](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/intro_0088CC.gif) `#0088CC` | ![0F88CA](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/pro_0F88CA.gif) `#0F88CA` |
//! |            12 |                 | ![0044CC](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/intro_0044CC.gif) `#0044CC` | ![16308B](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/pro_16308B.gif) `#16308B` |
//! |        **13** |               3 | ![0000CC](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/intro_0000CC.gif) `#0000CC` | ![173BA2](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/pro_173BA2.gif) `#173BA2` |
//! |            14 |                 | ![4400CC](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/intro_4400CC.gif) `#4400CC` | ![5C3F97](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/pro_5C3F97.gif) `#5C3F97` |
//! |            15 |               8 | ![8800CC](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/intro_8800CC.gif) `#8800CC` | ![6823B6](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/pro_6823B6.gif) `#6823B6` |
//! |            16 |               6 | ![CC00CC](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/intro_CC00CC.gif) `#CC00CC` | ![CE359E](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/pro_CE359E.gif) `#CE359E` |
//! |            17 |                 | ![CC0088](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/intro_CC0088.gif) `#CC0088` | ![DC1D49](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/pro_DC1D49.gif) `#DC1D49` |
//! |            18 |                 | ![CC0044](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/intro_CC0044.gif) `#CC0044` | ![C71136](https://raw.githubusercontent.com/Holzhaus/triseratops/main/assets/colors/pro_C71136.gif) `#C71136` |
//!
//! This means that if a Hotcue is saved with color `#C02626` in Serato DJ Pro, it actually saves
//! `#CC0000` in the metadata. If that file is opened in Serato DJ Intro, the hotcue will be
//! displayed with color `#CC0000` (i.e. the metadata color), if the file is opened in Serato DJ
//! Pro/Lite, the hotcue will be displayed with color `#C02626`.
//!
//! Hence, Serato DJ Intro is the only Serato DJ variant that displays the colors exactly like they
//! are saved (i.e. without applying any transformation/colorscheme).
//!
//! Note that Serato DJ Lite only has 4 hotcues with predefined, unchangeable colors (the
//! emphasized hotcue colors 1-4 in the table above).

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
