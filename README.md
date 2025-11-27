# 🛡️ Dev-Vault: Building a Developer's Code Snippet Manager in Rust

![Rust](https://img.shields.io/badge/Made_with-Rust-orange?style=for-the-badge&logo=rust)
![CLI](https://img.shields.io/badge/Type-CLI_Tool-blue?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)

> **Objective:** To learn Rust's ownership model, file I/O, and CLI ecosystem by building a persistent tool that helps developers recall complex terminal commands.

---

## 1. Title & Objective

* **Title:** Dev-Vault: Building a Developer's Code Snippet Manager in Rust
* **The Problem:** "Context Switching." Leaving the terminal to Google a command breaks flow and wastes time.
* **The Solution:** A persistent CLI tool to cache, search, and instantly retrieve terminal commands without leaving the terminal.

### **Key Features Implemented**
* ⚡ **Zero-Friction Retrieval:** Copy commands directly to system clipboard.
* 🔍 **Smart Search:** Fuzzy search through keys, descriptions, and tags.
* 💾 **Persistent Storage:** Global JSON database support.
* 🎨 **Beautiful UI:** Clean ASCII tables.

---

## 2. Quick Summary of the Technology

* **Technology:** **Rust** 🦀
* **What is it?** A systems programming language that guarantees memory safety and thread safety without requiring a garbage collector. It is known for its high performance and strict compiler.
* **Real-world example:** Used by **Discord** (for their high-performance backend services) and **Mozilla** (to build the Firefox browser engine).

---

## 3. System Requirements

* ✅ **OS:** Windows (running WSL/Ubuntu) or Linux.
* ✅ **Tools:** VS Code with the `rust-analyzer` extension.
* ✅ **Packages:** `cargo` (Rust's package manager and build system).

---

## 4. Installation & Setup Instructions

### Step 1: Installing Rust
Run the following command in your terminal to download the official installer:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  ` ``` `
This line downloads the script and initiates the setup.

When prompted, type 1 (for "Proceed with installation (default)") and hit Enter.

### Step 2: Configure Your Shell
Once the installation says "Rust is installed now. Great!", close your terminal window completely and open a new one to refresh your system PATH.

Step 3: Verify It Works
Type this to ensure the compiler is ready:

Bash

cargo --version
You should see output similar to: cargo 1.91.1 (xxxx-xx-xx)

Step 4: Create Your Project
Bash

cargo new dev-vault
cd dev-vault
Project Structure:

src/main.rs: Where our code logic lives.

Cargo.toml: The configuration file for dependencies.

Implementation
Step 1: Add the "Ingredients" (Dependencies)
We need specific external libraries to handle the CLI, JSON, and Clipboard interactions.

Action:

Open Cargo.toml.

Locate [dependencies].

Paste the following configuration:

Ini, TOML

[dependencies]
clap = { version = "4.5", features = ["derive"] }    # CLI Argument Parsing
serde = { version = "1.0", features = ["derive"] }   # Serialization
serde_json = "1.0"                                   # JSON Reading/Writing
comfy-table = "7.0"                                  # ASCII Tables
arboard = "3.2"                                      # Clipboard Management
colored = "2.0"                                      # Terminal Colors
dirs = "5.0"                                         # Home Directory Lookup
Step 2: Build
Run the build command to download and compile these tools:

Bash

cargo build
5. AI Prompt Journal
Prompt 1: Setting up the Core Structs
My Prompt:

"I have set up my Rust project with clap and serde. I need to create the main data structure. Please write the code for src/main.rs that defines a Snippet struct and uses clap to define a CLI with two subcommands: Add and List. Please explain what #[derive(Parser)] does."

🤖 AI Summary: The AI generated the Snippet struct and the Commands enum. It explained that #[derive(Parser)] is a macro that automatically implements command-line parsing logic based on the struct fields.

💡 Reflection: I learned about Rust's "Ownership" model specifically through .clone(). We had to clone string data to move it from the CLI arguments into our new Struct because Rust does not allow two variables to own the same memory simultaneously.

Prompt 2: File Handling (Load/Save)
My Prompt:

"The CLI is working! Now I need to replace the TODO comments with real file handling code. Please give me two helper functions: load_snippets() and save_snippets()."

🤖 AI Summary: The AI provided helper functions using BufReader and BufWriter for performance. It utilized serde_json::to_writer_pretty so the output file is human-readable.

💡 Reflection: I learned that BufReader is faster because it reduces the number of times the program asks the hard drive for data. I also saw how Result and .expect() are used to handle potential file errors safely.

Prompt 3: UI Improvements
My Prompt:

"I would like you to show me how to display the snippets in a pretty table using the comfy_table crate because the current output is ugly."

🤖 AI Summary: The AI instructed me to add comfy-table to Cargo.toml. It provided code to initialize a Table, load the UTF8_FULL preset (for rounded borders), and loop through the data to add rows.

💡 Reflection: I learned how external crates can instantly upgrade the UI of a terminal app. This makes the tool feel like professional software rather than a basic script.

Prompt 4: The 'Get' Command & Clipboard
My Prompt:

"I want to implement the Get command. It should take a key, find the snippet, and copy the command field directly to my system clipboard."

🤖 AI Summary: The AI suggested the arboard crate for cross-platform clipboard support. It showed me how to find a specific item in a Vector using .iter().find() and how to copy string data to the OS buffer.

💡 Reflection: I learned that interacting with the OS (like the clipboard) requires specific system libraries, but Rust creates abstract wrappers so the code looks the same on Windows, Linux, and Mac.

Prompt 5: Search Implementation
My Prompt:

"Please show me the Search variant for the Enum. It should take a query string and search the key, description, AND tags."

🤖 AI Summary: The AI demonstrated how to use Rust Iterators and Closures (filter(|s| ... )) to efficiently scan through the data. It used to_lowercase() to ensure the search was case-insensitive.

💡 Reflection: This showed the power of functional programming in Rust. Instead of writing a long for loop with many if statements, the logic was condensed into a clean iterator chain.

Prompt 6: Delete & Refactoring
My Prompt:

"Add the Delete command using retain to remove items from the vector."

🤖 AI Summary: The AI taught me how to use retain (which modifies the vector in-place) to remove items that match the key.

💡 Reflection: I now have a full CRUD application. I realized that handling data in this simple architecture requires loading the whole file into memory, modifying the vector, and saving it back—a pattern that works well for small datasets.

6. Common Issues & Fixes
The most challenging part of the project was graduating from a "local script" to a "global tool."

Challenge 1: Path Independence
Initially, the data was locked inside the project folder. If I navigated to another directory, the tool showed an empty list.

The Fix: I refactored the architecture to use the Home Directory for storage.

Persistence: Used the dirs crate to locate the user's home dynamically.

Storage: The database (.dev-vault.json) is now stored globally in the user's root.

Result: I can open a terminal in /var/www or /tmp and dev-vault still finds my data.

Challenge 2: Clipboard on Linux/WSL
When running the tool in WSL (Windows Subsystem for Linux), the clipboard function initially crashed or did nothing.

The Error: "Failed to initialize clipboard" / "X11 not found".

The Fix: I had to install the necessary Linux backend tools (sudo apt install xclip) so that the Rust binary could talk to the system clipboard.

Challenge 3: Table Layout Breaking
Long commands (like complex ffmpeg strings) broke the ASCII table layout, making it unreadable.

The Fix: I implemented ColumnConstraints using the comfy-table library to force wrapping on the Description and Command columns.

7. References
The Rust Programming Language (The Book)

Clap Documentation (CLI Builder)

Serde JSON Docs

Rust by Example (File I/O)
