use crate::ui::theme::{BuiltinThemeProvider, Theme, ThemeProvider};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiPreferences {
    pub theme: String,
    pub use_emojis: bool,
    pub color_output: ColorOutputMode,
}

impl Default for UiPreferences {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            use_emojis: true,
            color_output: ColorOutputMode::Auto,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ColorOutputMode {
    Auto,
    Always,
    Never,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UiConfig {
    pub preferences: UiPreferences,
    #[serde(default)]
    pub custom_themes: Vec<Theme>,
}

impl UiConfig {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .with_context(|| format!("Failed to read UI config from {config_path:?}"))?;

            let mut config: Self = serde_yaml::from_str(&content)
                .with_context(|| format!("Failed to parse UI config from {config_path:?}"))?;

            config.apply_env_overrides();
            Ok(config)
        } else {
            let mut config = Self::default();
            config.apply_env_overrides();
            Ok(config)
        }
    }

    #[allow(dead_code)]
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory {parent:?}"))?
        }

        let content = serde_yaml::to_string(self).context("Failed to serialize UI config")?;

        fs::write(&config_path, content)
            .with_context(|| format!("Failed to write UI config to {config_path:?}"))?;

        Ok(())
    }

    pub fn get_theme(&self) -> Theme {
        let provider = BuiltinThemeProvider;

        if let Some(theme) = self
            .custom_themes
            .iter()
            .find(|t| t.name.to_lowercase() == self.preferences.theme.to_lowercase())
        {
            return theme.clone();
        }

        if let Some(theme) = provider.get_theme(&self.preferences.theme) {
            return theme;
        }

        if let Ok(terminal_theme) = crate::ui::utils::detect_terminal_theme() {
            if terminal_theme {
                Theme::light()
            } else {
                Theme::dark()
            }
        } else {
            Theme::default()
        }
    }

    #[allow(dead_code)]
    pub fn should_use_color(&self) -> bool {
        match self.preferences.color_output {
            ColorOutputMode::Always => true,
            ColorOutputMode::Never => false,
            ColorOutputMode::Auto => {
                if std::env::var("NO_COLOR").is_ok() {
                    false
                } else if std::env::var("FORCE_COLOR").is_ok() {
                    true
                } else {
                    is_terminal::IsTerminal::is_terminal(&std::io::stdout())
                }
            }
        }
    }

    fn config_path() -> Result<PathBuf> {
        let home = dirs::home_dir().context("Failed to determine home directory")?;
        Ok(home.join(".swissarmyhammer").join("ui.yaml"))
    }

    fn apply_env_overrides(&mut self) {
        if let Ok(theme) = std::env::var("SAH_THEME") {
            self.preferences.theme = theme;
        }

        if let Ok(use_emojis) = std::env::var("SAH_USE_EMOJIS") {
            if let Ok(val) = use_emojis.parse::<bool>() {
                self.preferences.use_emojis = val;
            }
        }

        if std::env::var("NO_COLOR").is_ok() {
            self.preferences.color_output = ColorOutputMode::Never;
        } else if std::env::var("FORCE_COLOR").is_ok() {
            self.preferences.color_output = ColorOutputMode::Always;
        }
    }
}
