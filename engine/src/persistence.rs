use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub state: State,
    pub category: Category,
    pub priority: Priority,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum State {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Category {
    Work,
    Home,
    Study,
    SelfCare,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Priority {
    Highest,
    Higher,
    Lower,
    Lowest,
}

pub fn write_csv(tasks: Vec<Task>, path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    for task in tasks {
        wtr.serialize(task)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn read_csv(path: &str) -> Result<Vec<Task>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(File::open(path)?);
    let mut v: Vec<Task> = Vec::new();

    for result in rdr.deserialize() {
        let record: Task = result?;
        v.push(record);
    }
    Ok(v)
}
