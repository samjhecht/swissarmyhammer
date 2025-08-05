use anyhow::Result;
use std::env;

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum Icon {
    Success,
    Error,
    Warning,
    Info,
    Arrow,
    Bullet,
    Check,
    Cross,
    Question,
    Search,
    Folder,
    File,
    Lock,
    Unlock,
    Star,
    Heart,
    Fire,
    Lightning,
    Sparkles,
    Rocket,
}

pub mod icon {
    use super::Icon;
    use std::env;

    pub fn get(icon: Icon, use_emojis: bool) -> String {
        if use_emojis && supports_unicode() {
            match icon {
                Icon::Success => "✓",
                Icon::Error => "✗",
                Icon::Warning => "⚠",
                Icon::Info => "ℹ",
                Icon::Arrow => "→",
                Icon::Bullet => "•",
                Icon::Check => "✓",
                Icon::Cross => "✗",
                Icon::Question => "?",
                Icon::Search => "🔍",
                Icon::Folder => "📁",
                Icon::File => "📄",
                Icon::Lock => "🔒",
                Icon::Unlock => "🔓",
                Icon::Star => "⭐",
                Icon::Heart => "❤",
                Icon::Fire => "🔥",
                Icon::Lightning => "⚡",
                Icon::Sparkles => "✨",
                Icon::Rocket => "🚀",
            }
        } else {
            match icon {
                Icon::Success => "[OK]",
                Icon::Error => "[X]",
                Icon::Warning => "[!]",
                Icon::Info => "[i]",
                Icon::Arrow => "->",
                Icon::Bullet => "*",
                Icon::Check => "[v]",
                Icon::Cross => "[x]",
                Icon::Question => "[?]",
                Icon::Search => "[S]",
                Icon::Folder => "[D]",
                Icon::File => "[F]",
                Icon::Lock => "[L]",
                Icon::Unlock => "[U]",
                Icon::Star => "[*]",
                Icon::Heart => "[<3]",
                Icon::Fire => "[!]",
                Icon::Lightning => "[!]",
                Icon::Sparkles => "[*]",
                Icon::Rocket => "[^]",
            }
        }
        .to_string()
    }

    fn supports_unicode() -> bool {
        if let Ok(term) = env::var("TERM") {
            !term.contains("dumb")
        } else if env::var("WT_SESSION").is_ok() || env::var("TERMINAL_EMULATOR").is_ok() {
            true
        } else if cfg!(windows) {
            env::var("MSYSTEM").is_ok() || env::var("WT_SESSION").is_ok()
        } else {
            true
        }
    }
}

pub fn detect_terminal_theme() -> Result<bool> {
    if let Ok(colorfgbg) = env::var("COLORFGBG") {
        let parts: Vec<&str> = colorfgbg.split(';').collect();
        if parts.len() >= 2 {
            if let Ok(bg) = parts[1].parse::<u8>() {
                return Ok(bg >= 7);
            }
        }
    }

    if let Ok(term_program) = env::var("TERM_PROGRAM") {
        match term_program.as_str() {
            "iTerm.app" => {
                if let Ok(iterm_profile) = env::var("ITERM_PROFILE") {
                    return Ok(iterm_profile.to_lowercase().contains("light"));
                }
            }
            "Apple_Terminal" => {
                return Ok(false);
            }
            _ => {}
        }
    }

    if env::var("WT_SESSION").is_ok() {
        return Ok(false);
    }

    Ok(false)
}

#[allow(dead_code)]
pub fn terminal_width() -> usize {
    if let Some((width, _)) = term_size::dimensions() {
        width
    } else {
        80
    }
}

#[allow(dead_code)]
pub fn terminal_height() -> usize {
    if let Some((_, height)) = term_size::dimensions() {
        height
    } else {
        24
    }
}

#[allow(dead_code)]
pub fn truncate_with_ellipsis(text: &str, max_width: usize) -> String {
    if text.len() <= max_width {
        text.to_string()
    } else if max_width <= 3 {
        "...".to_string()
    } else {
        format!("{}...", &text[..max_width - 3])
    }
}

#[allow(dead_code)]
pub fn center_text(text: &str, width: usize) -> String {
    let text_width = text.chars().count();
    if text_width >= width {
        text.to_string()
    } else {
        let padding = (width - text_width) / 2;
        format!(
            "{}{}{}",
            " ".repeat(padding),
            text,
            " ".repeat(width - text_width - padding)
        )
    }
}

#[allow(dead_code)]
pub fn wrap_text(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![text.to_string()];
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;

    for word in text.split_whitespace() {
        let word_width = word.chars().count();

        if current_width > 0 && current_width + 1 + word_width > width {
            lines.push(current_line.trim().to_string());
            current_line = word.to_string();
            current_width = word_width;
        } else {
            if current_width > 0 {
                current_line.push(' ');
                current_width += 1;
            }
            current_line.push_str(word);
            current_width += word_width;
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line.trim().to_string());
    }

    if lines.is_empty() {
        vec![String::new()]
    } else {
        lines
    }
}
