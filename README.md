## Brief

The Objective: Build a modular project that separates concerns using Rust’s file-system-based module hierarchy (mod.rs vs. file-as-module), leverages external crates, and demonstrates how to expose a clean API.

The Project: "Task-Registry"

Build a CLI application that manages a list of tasks. Unlike fin-track, this project focuses on internal architecture and code organization.

The Requirements:
1. Project Structure:
  * Create a workspace or a standard library-based project structure.
  * Separate your core logic (The "Engine") into a src/lib.rs.
  * Create a src/commands/ directory to house your CLI command logic, using submodules (e.g., mod add, mod list, mod delete).
  * Use pub use statements in your lib.rs to re-export the most important types to make the API cleaner for the user.
2. Module Privacy:
  * Hide internal implementation details (use pub(crate) or private modules) so that the CLI and the library logic are strictly separated.
3. External Crates:
  * Use serde (with serde_json) for data persistence.
  * Use clap (the standard for Rust CLI apps) to handle command-line arguments.
  * Use anyhow for idiomatic error handling across your modules.
4. The "Challenge" Twist:
  * The Shared Utility Module: Create a src/utils module that is shared by both your CLI and your lib.rs. It must contain at least one helper function that is private to the project but accessible by all modules within the crate.
  * External Integration: Create a small "feature flag" in your Cargo.toml. If the feature verbose is enabled, print additional debug info in your library—demonstrating conditional compilation and feature management.

Why this tests your knowledge:
Modules: You will learn how mod declarations interact with the file system.
Encapsulation: You will learn when to use pub, pub(crate), and private access.
Dependency Management: You will handle real-world dependencies via Cargo.toml.
Organization: You will stop writing everything in main.rs, which is the most critical step in becoming a "Rustacean."

Functionalities to simple cli:
* Create task
* Modify task 
* Read tasks (by category) 
* Delete tasks
* Tasks categories
* Tasks priorities
* Report 
