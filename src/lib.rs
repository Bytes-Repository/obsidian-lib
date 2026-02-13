use clap::{Parser, Subcommand};
use gtk4 as gtk;
use gtk::{Application, ApplicationWindow, Button, Entry, Label, AlertDialog};
use gtk::prelude::*;
use gtk::gio::Cancellable;
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
    let cli = Cli::parse();

    match cli.command {
        Commands::Message { text, level } => {
            let app = Application::new(Some("com.hackerlang.obsidian.message"), Default::default());
            let level = level.to_lowercase();
            app.connect_activate(move |app| {
                let window = ApplicationWindow::new(app);
                window.set_visible(false);
                let mut builder = AlertDialog::builder()
                    .modal(true)
                    .buttons(vec!["OK"]);
                if level == "warning" {
                    builder = builder.message("Warning");
                } else if level == "error" {
                    builder = builder.message("Error");
                } else {
                    builder = builder.message("Information");
                }
                let dialog = builder.detail(&text).build();
                let window_clone = window.clone();
                dialog.choose(Some(&window), None::<&Cancellable>, move |_| {
                    window_clone.close();
                });
            });
            app.run();
        }
        Commands::Input { prompt } => {
            let app = Application::new(Some("com.hackerlang.obsidian"), Default::default());
            let input_received = Arc::new(AtomicBool::new(false));
            let input_text = Arc::new(std::sync::Mutex::new(String::new()));
            let input_received_for_closure = input_received.clone();
            let input_text_for_closure = input_text.clone();
            app.connect_activate(move |app| {
                let window = ApplicationWindow::new(app);
                window.set_title(Some("Obsidian Input"));
                window.set_default_size(400, 200);
                let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
                let label = Label::new(Some(&prompt));
                let entry = Entry::new();
                let button = Button::with_label("Submit");
                let input_text_clone = input_text_for_closure.clone();
                let input_received_clone = input_received_for_closure.clone();
                let entry_clone = entry.clone();
                let window_clone = window.clone();
                button.connect_clicked(move |_| {
                    let text = entry_clone.text().to_string();
                    *input_text_clone.lock().unwrap() = text;
                    input_received_clone.store(true, Ordering::SeqCst);
                    window_clone.close();
                });
                vbox.append(&label);
                vbox.append(&entry);
                vbox.append(&button);
                window.set_child(Some(&vbox));
                window.present();
            });
            app.run();
            if input_received.load(Ordering::SeqCst) {
                println!("{}", *input_text.lock().unwrap());
            } else {
                println!("Input cancelled");
            }
        }
        Commands::Confirm { text } => {
            let app = Application::new(Some("com.hackerlang.obsidian.confirm"), Default::default());
            let response_arc = Arc::new(std::sync::Mutex::new(-1i32));
            let response_clone = response_arc.clone();
            app.connect_activate(move |app| {
                let window = ApplicationWindow::new(app);
                window.set_visible(false);
                let dialog = AlertDialog::builder()
                    .modal(true)
                    .buttons(vec!["No", "Yes"])
                    .cancel_button(0)
                    .default_button(1)
                    .message("Confirmation")
                    .detail(&text)
                    .build();
                let window_clone = window.clone();
                let response_clone2 = response_clone.clone();
                dialog.choose(Some(&window), None::<&Cancellable>, move |choice| {
                    let val = match choice {
                        Ok(v) => v,
                        Err(_) => -1,
                    };
                    *response_clone2.lock().unwrap() = val;
                    window_clone.close();
                });
            });
            app.run();
            let response = *response_arc.lock().unwrap();
            println!("{}", if response == 1 { "yes" } else { "no" });
        }
    }
}
