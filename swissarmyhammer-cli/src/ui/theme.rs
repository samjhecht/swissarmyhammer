use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    #[allow(dead_code)]
    pub fn to_ansi_256(self) -> u8 {
        if self.r == self.g && self.g == self.b {
            if self.r < 8 {
                16
            } else if self.r > 248 {
                231
            } else {
                232 + ((self.r - 8) / 10)
            }
        } else {
            16 + (36 * (self.r / 51)) + (6 * (self.g / 51)) + (self.b / 51)
        }
    }

    #[allow(dead_code)]
    pub fn to_ansi_16(self) -> u8 {
        let brightness = (self.r as u16 + self.g as u16 + self.b as u16) / 3;
        let is_bright = brightness > 127;

        let max_component = self.r.max(self.g).max(self.b);
        let min_component = self.r.min(self.g).min(self.b);
        let diff = max_component - min_component;

        if diff < 30 {
            if is_bright {
                7
            } else {
                0
            }
        } else if self.r >= 200 && self.g < 100 && self.b < 100 {
            // Bright red
            9
        } else if self.r > self.g && self.r > self.b {
            if is_bright {
                9
            } else {
                1
            }
        } else if self.g > self.r && self.g > self.b {
            if is_bright {
                10
            } else {
                2
            }
        } else if self.b > self.r && self.b > self.g {
            if is_bright {
                12
            } else {
                4
            }
        } else if self.r == self.g && self.r > self.b {
            if is_bright {
                11
            } else {
                3
            }
        } else if self.r == self.b && self.r > self.g {
            if is_bright {
                13
            } else {
                5
            }
        } else if is_bright {
            14
        } else {
            6
        }
    }

    #[allow(dead_code)]
    pub fn to_hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::new(r, g, b)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    pub primary: Color,
    pub secondary: Color,
    pub success: Color,
    pub error: Color,
    pub warning: Color,
    pub info: Color,
    pub background: Color,
    pub foreground: Color,
    pub muted: Color,
    pub accent: Color,
    pub header: Color,
    pub link: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub colors: ColorPalette,
    pub is_dark: bool,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            name: "Light".to_string(),
            is_dark: false,
            colors: ColorPalette {
                primary: Color::new(33, 150, 243),     // Blue
                secondary: Color::new(156, 39, 176),   // Purple
                success: Color::new(76, 175, 80),      // Green
                error: Color::new(244, 67, 54),        // Red
                warning: Color::new(255, 152, 0),      // Orange
                info: Color::new(0, 188, 212),         // Cyan
                background: Color::new(255, 255, 255), // White
                foreground: Color::new(33, 33, 33),    // Dark gray
                muted: Color::new(117, 117, 117),      // Medium gray
                accent: Color::new(255, 64, 129),      // Pink
                header: Color::new(33, 33, 33),        // Dark gray
                link: Color::new(33, 150, 243),        // Blue
            },
        }
    }

    pub fn dark() -> Self {
        Self {
            name: "Dark".to_string(),
            is_dark: true,
            colors: ColorPalette {
                primary: Color::new(100, 181, 246),    // Light blue
                secondary: Color::new(206, 147, 216),  // Light purple
                success: Color::new(129, 199, 132),    // Light green
                error: Color::new(239, 83, 80),        // Light red
                warning: Color::new(255, 183, 77),     // Light orange
                info: Color::new(77, 208, 225),        // Light cyan
                background: Color::new(18, 18, 18),    // Very dark gray
                foreground: Color::new(238, 238, 238), // Light gray
                muted: Color::new(158, 158, 158),      // Medium gray
                accent: Color::new(255, 112, 167),     // Light pink
                header: Color::new(255, 255, 255),     // White
                link: Color::new(100, 181, 246),       // Light blue
            },
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

pub trait ThemeProvider {
    fn get_theme(&self, name: &str) -> Option<Theme>;
    #[allow(dead_code)]
    fn list_themes(&self) -> Vec<String>;
}

pub struct BuiltinThemeProvider;

impl ThemeProvider for BuiltinThemeProvider {
    fn get_theme(&self, name: &str) -> Option<Theme> {
        match name.to_lowercase().as_str() {
            "light" => Some(Theme::light()),
            "dark" => Some(Theme::dark()),
            _ => None,
        }
    }

    fn list_themes(&self) -> Vec<String> {
        vec!["light".to_string(), "dark".to_string()]
    }
}

lazy_static::lazy_static! {
    pub static ref LIGHT_THEME: Theme = Theme::light();
    pub static ref DARK_THEME: Theme = Theme::dark();
}
