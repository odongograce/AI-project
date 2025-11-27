# 🛡️ Dev-Vault: The Developer's Snippet Manager
> "Stop Googling the same Git commands every week."

**Dev-Vault** is a lightning-fast, persistent Command Line Interface (CLI) tool built in **Rust**. It helps developers cache, search, and instantly retrieve complex terminal commands without leaving the terminal.
****Objective: **** To learn Rust's ownership model, file I/O, and CLI ecosystem by building a persistent tool that helps developers recall complex terminal commands.
## 🚀 Features

* **⚡ Zero-Friction Retrieval:** Copy commands directly to your system clipboard with `get`.

* **🔍 Smart Search:** Fuzzy search through keys, descriptions, and tags.

* **💾 Persistent Storage:** Saves your library to a local JSON database.

* **🏷️ Tagging System:** Organize snippets by technology (e.g., `git`, `docker`, `rust`).

* **🎨 Beautiful UI:** Renders data in clean, readable ASCII tables.


## 🛠️ Tech Stack

* **Language:** Rust 🦀

* **CLI Parsing:** `clap`

* **Serialization:** `serde` & `serde_json`

* **UI/Tables:** `comfy-table`

* **System Integration:** `arboard` (Clipboard management)

## 📦 Installation

### Prerequisites

* Rust (Cargo) installed.

### Build from Source

```bash

# 1. Clone the repository

git clone [https://github.com/yourusername/dev-vault.git](https://github.com/yourusername/dev-vault.git)


# 2. Navigate to the directory

cd dev-vault


# 3. Build the project

cargo build --release



📖 Usage Guide

1. Add a Snippet

Save a new command to your vault.


cargo run -- add --key git-undo --description "Undo last commit" --command "git reset --soft HEAD~1" --tags git,oops

2. List All Snippets

View your entire library in a formatted table.

cargo run -- list

3. Get to Clipboard (The Magic ✨)

Finds the snippet and automatically copies the command to your clipboard.

cargo run -- get git-undo

4. Search

Find commands even if you don't remember the exact key.

cargo run -- search undo


5. Delete

Remove outdated snippets.

cargo run -- delete git-undo


🧠 Project Architecture

This tool follows a modular architecture:

CLI Layer (clap): Parses user input and routes commands.

Logic Layer: Handles data manipulation (CRUD operations).

Persistence Layer: Serializes Rust structs to snippets.json
