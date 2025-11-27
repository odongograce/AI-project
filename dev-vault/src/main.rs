use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
// UPDATED IMPORT: Added Cell and Color back
use comfy_table::{Table, presets::UTF8_FULL, ContentArrangement, Cell, Color}; 
use arboard::Clipboard;
use colored::*; 

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
        key: String,
    },
    /// Search for snippets by keyword
    Search {
        keyword: String,
    },
    /// Delete a snippet
    Delete {
        key: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)] 
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

// FIX: Updated this function to use comfy-table's internal coloring
fn print_table(snippets: Vec<Snippet>) {
    if snippets.is_empty() {
        println!("{}", "No snippets found.".yellow());
        return;
    }
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    
    table.set_header(vec![
        "Key", "Description", "Command", "Tags"
    ]);

    for snippet in snippets {
        // We create Cells explicitly so we can style them safely
        table.add_row(vec![
            Cell::new(&snippet.key).fg(Color::Cyan),         // Safe Cyan
            Cell::new(&snippet.description),                 // Default color
            Cell::new(&snippet.command).fg(Color::Green),    // Safe Green
            Cell::new(&snippet.tags.join(", ")).fg(Color::Blue) // Safe Blue
        ]);
    }
    println!("{table}");
}

fn print_banner() {
    let art = r#"
  ____             _     __     __          _ _ 
 |  _ \  _____   _| |____\ \   / /_ _ _   _| | |_ 
 | | | |/ _ \ \ / / |_____\ \ / / _` | | | | | __|
 | |_| |  __/\ V /| |      \ V / (_| | |_| | | |_ 
 |____/ \___| \_/ |_|       \_/ \__,_|\__,_|_|\__|
                                                  
"#;
    println!("{}", art.bright_purple().bold());
    println!("{}", ">> The Developer's External Memory <<".italic().dimmed());
    println!("--------------------------------------------------");
}

fn main() {
    print_banner();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { key, description, command, tags } => {
            let mut snippets = load_snippets();
            if snippets.iter().any(|s| s.key == *key) {
                println!("{} A snippet with key '{}' already exists.", "❌ Error:".red().bold(), key);
                return;
            }
            let new_snippet = Snippet {
                key: key.clone(),
                description: description.clone(),
                command: command.clone(),
                tags: tags.clone(),
            };
            
            snippets.push(new_snippet.clone()); 
            
            save_snippets(&snippets);
            println!("{} Added snippet: {}", "✅ Success!".green().bold(), new_snippet.key.cyan());
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
                    println!("{} '{}'", "Found:".green(), s.description);
                    let mut clipboard = Clipboard::new().expect("Failed to init clipboard");
                    clipboard.set_text(&s.command).expect("Failed to copy");
                    println!("{} Copied command to clipboard!", "✅ Success!".green().bold());
                },
                None => println!("{} No snippet found with key: '{}'", "❌ Error:".red().bold(), key),
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
            
            if filtered_snippets.is_empty() {
                println!("{} No matches found for '{}'", "🔍 Info:".yellow(), keyword);
            } else {
                println!("{} Found {} matches:", "🔍 Search:".blue().bold(), filtered_snippets.len());
                print_table(filtered_snippets);
            }
        }
        Commands::Delete { key } => {
            let mut snippets = load_snippets();
            let initial_len = snippets.len();
            snippets.retain(|s| s.key != *key);
            
            if snippets.len() < initial_len {
                save_snippets(&snippets);
                println!("{} Deleted snippet: {}", "🗑️ Removed:".red().bold(), key.cyan());
            } else {
                println!("{} Snippet not found: {}", "❌ Error:".red().bold(), key);
            }
        }
    }
}