//! Example showing how to migrate from direct colored usage to the new UI system

use colored::*;
use swissarmyhammer_cli::ui::{Icon, UiContext};

fn old_way() {
    println!("\n=== Old Way (using colored directly) ===\n");

    println!("{}", "✓ Success message".green());
    println!("{}", "✗ Error message".red());
    println!("{}", "⚠ Warning message".yellow());
    println!("{}", "ℹ Info message".blue());

    println!("\n{}", "Header Text".bold());
    println!("{}", "Muted text".dimmed());
}

fn new_way() {
    println!("\n=== New Way (using UI system) ===\n");

    let ui = UiContext::default();

    println!(
        "{} {}",
        ui.icon(Icon::Success),
        ui.success("Success message")
    );
    println!("{} {}", ui.icon(Icon::Error), ui.error("Error message"));
    println!(
        "{} {}",
        ui.icon(Icon::Warning),
        ui.warning("Warning message")
    );
    println!("{} {}", ui.icon(Icon::Info), ui.info("Info message"));

    println!("\n{}", ui.header("Header Text"));
    println!("{}", ui.muted("Muted text"));
}

fn advanced_features() {
    println!("\n=== Advanced Features ===\n");

    let ui = UiContext::default();

    // Theme-aware coloring
    println!("{}", ui.primary("Primary color adapts to theme"));
    println!("{}", ui.secondary("Secondary color for supporting content"));

    // Consistent icons with fallback
    println!("{} Searching...", ui.icon(Icon::Search));
    println!("{} Processing directory", ui.icon(Icon::Folder));

    // Text utilities
    use swissarmyhammer_cli::ui::utils::wrap_text;
    let long_text = "This demonstrates the text wrapping utility that can be used for better formatting of long messages in the terminal";
    let wrapped = wrap_text(long_text, 50);
    for line in wrapped {
        println!("  {}", ui.muted(&line));
    }
}

fn main() {
    old_way();
    new_way();
    advanced_features();

    println!("\n=== Benefits of the new system ===");
    println!("• Consistent theming across the application");
    println!("• Automatic dark/light mode detection");
    println!("• Semantic colors that adapt to themes");
    println!("• Graceful emoji/icon fallbacks");
    println!("• User preferences persistence");
    println!("• Better accessibility support");
}
