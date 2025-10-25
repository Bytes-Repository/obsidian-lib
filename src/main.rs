use clap::{Parser, Subcommand};
use gtk4 as gtk;
use gtk::{Application, ApplicationWindow, Button, Dialog, Entry, Label, MessageType, ButtonsType};
use gtk::prelude::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

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
    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK");

    let cli = Cli::parse();

    match cli.command {
        Commands::Message { text, level } => {
            let msg_type = match level.to_lowercase().as_str() {
                "warning" => MessageType::Warning,
                "error" => MessageType::Error,
                _ => MessageType::Info,
            };
            let dialog = gtk::MessageDialog::new(
                None::<&ApplicationWindow>,
                gtk::DialogFlags::MODAL,
                msg_type,
                ButtonsType::Ok,
                &text,
            );
            dialog.set_title("Obsidian Message");
            dialog.run();
            dialog.close();
        }
        Commands::Input { prompt } => {
            let app = Application::new(Some("com.hackerlang.obsidian"), Default::default());
            let input_received = Arc::new(AtomicBool::new(false));
            let input_text = Arc::new(std::sync::Mutex::new(String::new()));

            app.connect_activate(move |app| {
                let window = ApplicationWindow::new(app);
                window.set_title(Some("Obsidian Input"));
                window.set_default_size(400, 200);

                let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
                let label = Label::new(Some(&prompt));
                let entry = Entry::new();
                let button = Button::with_label("Submit");

                let input_text_clone = input_text.clone();
                let input_received_clone = input_received.clone();
                button.connect_clicked(move |_| {
                    let text = entry.text().to_string();
                    *input_text_clone.lock().unwrap() = text;
                    input_received_clone.store(true, Ordering::SeqCst);
                    window.close();
                });

                vbox.append(&label);
                vbox.append(&entry);
                vbox.append(&button);
                window.set_child(Some(&vbox));
                window.present();
            });

            app.run();
            if input_received.load(Ordering::SeqCst) {
                println!("{}", input_text.lock().unwrap());
            } else {
                println!("Input cancelled");
            }
        }
        Commands::Confirm { text } => {
            let dialog = gtk::MessageDialog::new(
                None::<&ApplicationWindow>,
                gtk::DialogFlags::MODAL,
                MessageType::Question,
                ButtonsType::YesNo,
                &text,
            );
            dialog.set_title("Obsidian Confirm");
            let response = dialog.run();
            dialog.close();
            println!("{}", if response == gtk::ResponseType::Yes { "yes" } else { "no" });
        }
    }
}
