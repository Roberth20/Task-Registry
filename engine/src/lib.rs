pub mod create;
pub mod delete;
pub mod modify;
mod persistence;
pub mod read;
pub mod report;

use persistence::{read_csv, write_csv};

pub read(file_path: &str) -> Result 

pub fn example_write() {
    use persistence::{Category, Priority, State, Task};

    let mut tasks: Vec<Task> = Vec::new();

    tasks.push(Task {
        name: "Buy fruit".to_string(),
        description: "There are not more fruit. I have to buy 2 Kilograms of oranges".to_string(),
        state: State::ToDo,
        category: Category::Home,
        priority: Priority::Lower,
    });

    tasks.push(Task {
        name: "Work".to_string(),
        description: "The banks send the statements, they must be validated".to_string(),
        state: State::InProgress,
        category: Category::Work,
        priority: Priority::Higher,
    });

    tasks.push(Task {
        name: "Wash my hair".to_string(),
        description: "It's really dirty because last day run".to_string(),
        state: State::Done,
        category: Category::SelfCare,
        priority: Priority::Highest,
    });

    write_csv(tasks, "test.csv");
    let data = read_csv("test.csv");
    println!("{:.?}", data);
}
