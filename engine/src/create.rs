use crate::persistence::{Category, Priority, State, Task, read_csv, write_csv};
use std::error::Error;

pub fn add_task(
    file_path: &str,
    name: String,
    description: String,
    state: State,
    category: Category,
    priority: Priority,
) -> Result<(), Box<dyn Error>> {
    let mut tasks = match read_csv(file_path) {
        Ok(vec) => vec,
        Err(e) => {
            println!("The file does not exists! Creating new registry.");
            Vec::new()
        }
    };

    tasks.push(Task {
        name,
        description,
        state,
        category,
        priority,
    });
    write_csv(tasks, file_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_task() {
        let out = add_task(
            "../test.csv",
            "Buy fruit".to_string(),
            "I want apples".to_string(),
            State::ToDo,
            Category::Home,
            Priority::Lower,
        );
        dbg!("{:.?}", &out);
        assert!(out.is_ok());
    }
}
