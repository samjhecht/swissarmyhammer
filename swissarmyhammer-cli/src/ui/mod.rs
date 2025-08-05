pub mod config;
pub mod style;
pub mod theme;
pub mod utils;

pub use config::UiConfig;
pub use style::{Style, StyledText};
#[allow(unused_imports)]
pub use theme::Color;
pub use theme::{Theme, ThemeProvider};
pub use utils::{icon, Icon};

use anyhow::Result;
use std::sync::Arc;

#[derive(Clone)]
pub struct UiContext {
    theme: Arc<Theme>,
    config: Arc<UiConfig>,
}

impl UiContext {
    pub fn new() -> Result<Self> {
        let config = UiConfig::load()?;
        let theme = config.get_theme();

        Ok(Self {
            theme: Arc::new(theme),
            config: Arc::new(config),
        })
    }

    pub fn with_theme(theme: Theme) -> Self {
        let config = UiConfig::default();

        Self {
            theme: Arc::new(theme),
            config: Arc::new(config),
        }
    }

    #[allow(dead_code)]
    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    #[allow(dead_code)]
    pub fn config(&self) -> &UiConfig {
        &self.config
    }

    pub fn style(&self) -> Style {
        Style::new(self.theme.clone())
    }

    #[allow(dead_code)]
    pub fn primary(&self, text: impl Into<String>) -> StyledText {
        self.style().primary(text)
    }

    #[allow(dead_code)]
    pub fn secondary(&self, text: impl Into<String>) -> StyledText {
        self.style().secondary(text)
    }

    pub fn success(&self, text: impl Into<String>) -> StyledText {
        self.style().success(text)
    }

    pub fn error(&self, text: impl Into<String>) -> StyledText {
        self.style().error(text)
    }

    pub fn warning(&self, text: impl Into<String>) -> StyledText {
        self.style().warning(text)
    }

    pub fn info(&self, text: impl Into<String>) -> StyledText {
        self.style().info(text)
    }

    pub fn muted(&self, text: impl Into<String>) -> StyledText {
        self.style().muted(text)
    }

    pub fn header(&self, text: impl Into<String>) -> StyledText {
        self.style().header(text)
    }

    #[allow(dead_code)]
    pub fn accent(&self, text: impl Into<String>) -> StyledText {
        self.style().accent(text)
    }

    pub fn icon(&self, icon: Icon) -> String {
        icon::get(icon, self.config.preferences.use_emojis)
    }
}

impl Default for UiContext {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self::with_theme(Theme::default()))
    }
}
