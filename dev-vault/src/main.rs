use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf; // Changed from Path to PathBuf
use comfy_table::{Table, presets::UTF8_FULL, ContentArrangement, Cell, Color, Width, ColumnConstraint}; 
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
    Add {
        #[arg(short, long)]
        key: String,
        #[arg(short, long, alias = "desc")] 
        description: String,
        #[arg(short, long)]
        command: String,
        #[arg(short, long, value_delimiter = ',', num_args = 1..)]
        tags: Vec<String>,
    },
    List,
    Get { key: String },
    Search { keyword: String },
    Delete { key: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)] 
pub struct Snippet {
    pub key: String,
    pub description: String,
    pub command: String,
    pub tags: Vec<String>,
}

// --- UPDATED: GLOBAL FILE PATH ---

fn get_db_path() -> PathBuf {
    // This finds your home directory (e.g., /home/odongo/)
    let mut path = dirs::home_dir().expect("Could not find home directory");
    // It saves the file as hidden: /home/odongo/.dev-vault.json
    path.push(".dev-vault.json");
    path
}

fn load_snippets() -> Vec<Snippet> {
    let path = get_db_path();
    if !path.exists() {
        return Vec::new(); 
    }
    let file = File::open(path).expect("Failed to open database file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
}

fn save_snippets(snippets: &Vec<Snippet>) {
    let path = get_db_path();
    let file = File::create(path).expect("Failed to create database file");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, snippets).expect("Failed to write to database");
}

// --- DISPLAY LOGIC ---

fn print_table(snippets: Vec<Snippet>) {
    if snippets.is_empty() {
        println!("{}", "No snippets found.".yellow());
        return;
    }
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    
    table.set_constraints(vec![
        ColumnConstraint::LowerBoundary(Width::Fixed(10)),      
        ColumnConstraint::UpperBoundary(Width::Percentage(30)), 
        ColumnConstraint::UpperBoundary(Width::Percentage(40)), 
        ColumnConstraint::UpperBoundary(Width::Percentage(20)), 
    ]);

    table.set_header(vec![
        Cell::new("Key").fg(Color::Green),
        Cell::new("Description").fg(Color::Green),
        Cell::new("Command").fg(Color::Green),
        Cell::new("Tags").fg(Color::Green),
    ]);

    for snippet in snippets {
        table.add_row(vec![
            Cell::new(&snippet.key).fg(Color::Cyan),         
            Cell::new(&snippet.description),                 
            Cell::new(&snippet.command).fg(Color::Yellow),    
            Cell::new(&snippet.tags.join(", ")).fg(Color::DarkGrey) 
        ]);
    }
    println!("{table}");
}

fn print_banner() {
    let art = r#"
  ____             _     _   __            _ _ 
 |  _ \  _____   _| |___ \ \/ /_ _ _   _lt| | |_ 
 | | | |/ _ \ \ / / / __| \  / _` | | | | | __|
 | |_| |  __/\ V /| \__ \ /  \ (_| | |_| | | |_ 
 |____/ \___| \_/ |_|___//_/\_\__,_|\__,_|_|\__|
                                                
"#;
    println!("{}", art.bright_purple().bold());
    println!("{}", ">> The Developer's External Memory <<".italic().dimmed());
    println!("--------------------------------------------------");
}

fn main() {
    let cli = Cli::parse();

    // Only print banner for interactive commands
    if let Commands::Get { .. } = &cli.command {
        // quiet
    } else {
        print_banner();
    }

    match &cli.command {
        Commands::Add { key, description, command, tags } => {
            let mut snippets = load_snippets();
            if snippets.iter().any(|s| s.key == *key) {
                println!("{} Key '{}' already exists!", "❌ Error:".red().bold(), key);
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
            let filtered: Vec<Snippet> = snippets.into_iter()
                .filter(|s| 
                    s.key.to_lowercase().contains(&keyword_lower) ||
                    s.description.to_lowercase().contains(&keyword_lower) ||
                    s.tags.iter().any(|t| t.to_lowercase().contains(&keyword_lower))
                ).collect();
            
            if filtered.is_empty() {
                println!("{} No matches found for '{}'", "🔍 Info:".yellow(), keyword);
            } else {
                println!("{} Found {} matches:", "🔍 Search:".blue().bold(), filtered.len());
                print_table(filtered);
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