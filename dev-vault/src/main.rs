use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use comfy_table::{Table, presets::UTF8_FULL};
use arboard::Clipboard;

#[derive(Parser)]
#[command(name = "dev-vault")]
#[command(about = "A developer's CLI code snippet manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new snippet
    Add {
        #[arg(short, long)]
        key: String,
        #[arg(short, long)]
        description: String,
        #[arg(short, long)]
        command: String,
        #[arg(short, long, value_delimiter = ',', num_args = 1..)]
        tags: Vec<String>,
    },
    /// List all saved snippets
    List,
    /// Get a snippet and copy it to the clipboard
    Get {
        /// The key of the snippet to get
        key: String,
    },
    /// Search for snippets by keyword
    Search {
        /// The keyword to search for (checks key, description, and tags)
        keyword: String,
    },
    /// Delete a snippet
    Delete {
        /// The key of the snippet to delete
        key: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Snippet {
    pub key: String,
    pub description: String,
    pub command: String,
    pub tags: Vec<String>,
}

// --- HELPER FUNCTIONS ---

fn load_snippets() -> Vec<Snippet> {
    let path = Path::new("snippets.json");
    if !path.exists() {
        return Vec::new(); 
    }
    let file = File::open(path).expect("Failed to open snippets.json");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to parse snippets.json")
}

fn save_snippets(snippets: &Vec<Snippet>) {
    let file = File::create("snippets.json").expect("Failed to create snippets.json");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, snippets).expect("Failed to write to snippets.json");
}

fn print_table(snippets: Vec<Snippet>) {
    if snippets.is_empty() {
        println!("No snippets found.");
        return;
    }
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["Key", "Description", "Command", "Tags"]);

    for snippet in snippets {
        table.add_row(vec![
            &snippet.key,
            &snippet.description,
            &snippet.command,
            &snippet.tags.join(", ")
        ]);
    }
    println!("{table}");
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { key, description, command, tags } => {
            let mut snippets = load_snippets();
            // Check if key already exists
            if snippets.iter().any(|s| s.key == *key) {
                println!("Error: A snippet with key '{}' already exists.", key);
                return;
            }
            let new_snippet = Snippet {
                key: key.clone(),
                description: description.clone(),
                command: command.clone(),
                tags: tags.clone(),
            };
            println!("Added snippet: {}", new_snippet.key);
            snippets.push(new_snippet);
            save_snippets(&snippets);
        }
        Commands::List => {
            let snippets = load_snippets();
            print_table(snippets);
        }
        Commands::Get { key } => {
            let snippets = load_snippets();
            let snippet = snippets.iter().find(|s| s.key == *key);
            match snippet {
                Some(s) => {
                    println!("Found: '{}'", s.description);
                    let mut clipboard = Clipboard::new().expect("Failed to init clipboard");
                    clipboard.set_text(&s.command).expect("Failed to copy");
                    println!("Copied command to clipboard!");
                },
                None => println!(" No snippet found with key: '{}'", key),
            }
        }
        Commands::Search { keyword } => {
            let snippets = load_snippets();
            let keyword_lower = keyword.to_lowercase();
            let filtered_snippets: Vec<Snippet> = snippets
                .into_iter()
                .filter(|s| {
                    s.key.to_lowercase().contains(&keyword_lower) ||
                    s.description.to_lowercase().contains(&keyword_lower) ||
                    s.tags.iter().any(|t| t.to_lowercase().contains(&keyword_lower))
                })
                .collect();
            println!("Search results for '{}':", keyword);
            print_table(filtered_snippets);
        }
        Commands::Delete { key } => {
            let mut snippets = load_snippets();
            let initial_len = snippets.len();
            // Retain only snippets that DO NOT match the key
            snippets.retain(|s| s.key != *key);
            
            if snippets.len() < initial_len {
                save_snippets(&snippets);
                println!("Deleted snippet: {}", key);
            } else {
                println!("Snippet not found: {}", key);
            }
        }
    }
}