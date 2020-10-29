use colored::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Debug, Clone, PartialEq)]
pub enum Output {
    /// only to notify the output processing about a file, that was processed (does not show in CLI)
    None,
    Error(String),
    Warning(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct OutputMessage {
    pub file: Arc<PathBuf>,
    pub output: Output,
}

pub async fn handle_processing_output(mut rx: mpsc::Receiver<OutputMessage>) -> i32 {
    let mut map = HashMap::new();
    let mut file_count = 0;
    let mut warning_count = 0;
    let mut error_count = 0;

    while let Some(msg) = rx.recv().await {
        let entry = map.entry(msg.file).or_insert(vec![]);

        match msg.output {
            Output::None => {}
            _ => {
                entry.push(msg.output);
            }
        }
    }

    for (k, v) in map {
        file_count += 1;

        if v.len() == 0 {
            continue;
        }

        println!("\nFile: {:?}", k);

        for output in v {
            match output {
                Output::Error(message) => {
                    error_count += 1;
                    println!("[{}] {}", "Error", message.red());
                }
                Output::Warning(message) => {
                    warning_count += 1;
                    println!("[{}] {}", "Warning", message.yellow());
                }
                Output::None => {}
            }
        }
    }

    println!(
        "\nFiles scanned: {}, Errors: {}, Warnings: {}",
        file_count, error_count, warning_count
    );

    if file_count > 0 && (error_count > 0 || warning_count > 0) {
        println!("{}", "Happy bug fixing ;)".black().on_white());
        return 1;
    } else if file_count > 0 {
        println!("{}", "Good job! o.O".on_green());
        return 0;
    }

    return 0;
}
