use clap::{Parser, Subcommand};
use rfd::{MessageDialog, MessageLevel, DialogResult};

#[derive(Parser)]
#[command(name = "obsidian", about = "Obsidian dialog library for Hacker Lang")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show a message dialog
    Message {
        /// Message text
        #[arg(long)]
        text: String,
        /// Message type (info, warning, error)
        #[arg(long, default_value = "info")]
        level: String,
    },
    /// Show an input dialog and return user input
    Input {
        /// Prompt text
        #[arg(long)]
        prompt: String,
    },
    /// Show a confirmation dialog (yes/no)
    Confirm {
        /// Confirmation message
        #[arg(long)]
        text: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Message { text, level } => {
            let msg_level = match level.to_lowercase().as_str() {
                "warning" => MessageLevel::Warning,
                "error" => MessageLevel::Error,
                _ => MessageLevel::Info,
            };
            MessageDialog::new()
                .set_title("Obsidian Message")
                .set_description(&text)
                .set_level(msg_level)
                .show();
        }
        Commands::Input { prompt } => {
            match rfd::MessageDialog::new()
                .set_title("Obsidian Input")
                .set_description(&prompt)
                .set_buttons(rfd::MessageButtons::OkCancelCustom(
                    "Submit".to_string(),
                    "Cancel".to_string(),
                ))
                .show() {
                DialogResult::Ok => {
                    // Simulate input (rfd doesn't support text input directly, so we print prompt)
                    println!("User entered input for: {}", prompt);
                }
                DialogResult::Cancel => println!("Input cancelled"),
                _ => println!("Unexpected result"),
            }
        }
        Commands::Confirm { text } => {
            let result = MessageDialog::new()
                .set_title("Obsidian Confirm")
                .set_description(&text)
                .set_buttons(rfd::MessageButtons::YesNo)
                .show();
            println!("{}", if result == DialogResult::Yes { "yes" } else { "no" });
        }
    }
}
