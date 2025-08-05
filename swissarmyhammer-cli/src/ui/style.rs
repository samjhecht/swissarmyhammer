use crate::ui::theme::{Color, Theme};
use colored::{ColoredString, Colorize};
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum TextDecoration {
    Bold,
    Italic,
    Underline,
    Dimmed,
    Reversed,
    Strikethrough,
}

#[derive(Clone)]
pub struct Style {
    theme: Arc<Theme>,
}

impl Style {
    pub fn new(theme: Arc<Theme>) -> Self {
        Self { theme }
    }

    #[allow(dead_code)]
    pub fn primary(&self, text: impl Into<String>) -> StyledText {
        StyledText::new(text.into(), Some(self.theme.colors.primary), None)
    }

    #[allow(dead_code)]
    pub fn secondary(&self, text: impl Into<String>) -> StyledText {
        StyledText::new(text.into(), Some(self.theme.colors.secondary), None)
    }

    pub fn success(&self, text: impl Into<String>) -> StyledText {
        StyledText::new(text.into(), Some(self.theme.colors.success), None)
    }

    pub fn error(&self, text: impl Into<String>) -> StyledText {
        StyledText::new(text.into(), Some(self.theme.colors.error), None)
    }

    pub fn warning(&self, text: impl Into<String>) -> StyledText {
        StyledText::new(text.into(), Some(self.theme.colors.warning), None)
    }

    pub fn info(&self, text: impl Into<String>) -> StyledText {
        StyledText::new(text.into(), Some(self.theme.colors.info), None)
    }

    pub fn muted(&self, text: impl Into<String>) -> StyledText {
        StyledText::new(text.into(), Some(self.theme.colors.muted), None)
    }

    pub fn header(&self, text: impl Into<String>) -> StyledText {
        StyledText::new(text.into(), Some(self.theme.colors.header), None).bold()
    }

    #[allow(dead_code)]
    pub fn link(&self, text: impl Into<String>) -> StyledText {
        StyledText::new(text.into(), Some(self.theme.colors.link), None).underline()
    }

    #[allow(dead_code)]
    pub fn accent(&self, text: impl Into<String>) -> StyledText {
        StyledText::new(text.into(), Some(self.theme.colors.accent), None)
    }
}

#[derive(Clone)]
pub struct StyledText {
    text: String,
    foreground: Option<Color>,
    background: Option<Color>,
    decorations: Vec<TextDecoration>,
}

#[allow(dead_code)]
impl StyledText {
    pub fn new(text: String, foreground: Option<Color>, background: Option<Color>) -> Self {
        Self {
            text,
            foreground,
            background,
            decorations: Vec::new(),
        }
    }

    pub fn plain(text: impl Into<String>) -> Self {
        Self::new(text.into(), None, None)
    }

    pub fn fg(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    pub fn bold(mut self) -> Self {
        self.decorations.push(TextDecoration::Bold);
        self
    }

    pub fn italic(mut self) -> Self {
        self.decorations.push(TextDecoration::Italic);
        self
    }

    pub fn underline(mut self) -> Self {
        self.decorations.push(TextDecoration::Underline);
        self
    }

    pub fn dimmed(mut self) -> Self {
        self.decorations.push(TextDecoration::Dimmed);
        self
    }

    pub fn reversed(mut self) -> Self {
        self.decorations.push(TextDecoration::Reversed);
        self
    }

    pub fn strikethrough(mut self) -> Self {
        self.decorations.push(TextDecoration::Strikethrough);
        self
    }

    pub fn render(&self) -> ColoredString {
        let mut result = ColoredString::from(self.text.as_str());

        if let Some(fg) = self.foreground {
            result = apply_foreground_color(result, fg);
        }

        if let Some(bg) = self.background {
            result = apply_background_color(result, bg);
        }

        for decoration in &self.decorations {
            result = apply_decoration(result, *decoration);
        }

        result
    }
}

impl fmt::Display for StyledText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

fn apply_foreground_color(text: ColoredString, color: Color) -> ColoredString {
    text.truecolor(color.r, color.g, color.b)
}

fn apply_background_color(text: ColoredString, color: Color) -> ColoredString {
    text.on_truecolor(color.r, color.g, color.b)
}

fn apply_decoration(text: ColoredString, decoration: TextDecoration) -> ColoredString {
    match decoration {
        TextDecoration::Bold => text.bold(),
        TextDecoration::Italic => text.italic(),
        TextDecoration::Underline => text.underline(),
        TextDecoration::Dimmed => text.dimmed(),
        TextDecoration::Reversed => text.reversed(),
        TextDecoration::Strikethrough => text.strikethrough(),
    }
}

#[allow(dead_code)]
pub fn adapt_colored_migration(colored_string: &str) -> StyledText {
    StyledText::plain(colored_string)
}
