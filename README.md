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

Step 1: Installing Rust
Run the following command in your terminal to download the official installer:
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh

This line downloads the script and initiates the setup.

When prompted, type 1 (for "Proceed with installation (default)") and hit Enter.

Step 2: Configure Your Shell
Once the installation says "Rust is installed now. Great!", close your terminal window completely and open a new one to refresh your system PATH.

Step 3: Verify It Works
Type this to ensure the compiler is ready:

cargo --version
