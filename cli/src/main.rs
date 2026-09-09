use clap::{Parser, Subcommand};

/// Interface for task registry application
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Operation 
    #[command(subcommand)]
    operation: Operation,
}

/// Sub-commands of the operations
#[derive(Subcommand, Debug)]
enum Operation {
    /// Create a new task 
    Create,
    /// Modify existing task
    Modify,
    /// List the current tasks
    Read,
    /// Delete a task
    Delete,
    /// Build a report of current tasks state
    Report,
}

fn main() {
    let cli = Cli::parse();

    match cli.operation {
        Operation::Create => {
            println!("You selected create a task!");
            engine::example_write();
        },
        Operation::Modify => println!("You selected modify a task"),
        Operation::Read => println!("Let's check your tasks"),
        Operation::Delete => println!("Marking task to deletion"),
        Operation::Report => println!("Building report")
    };
}
