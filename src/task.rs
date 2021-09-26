use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Error,ErrorKind, Seek, SeekFrom};
use std::path::PathBuf;
use std::fmt;



#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl fmt::Display for Task{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}

impl Task {
    pub fn new(text: String) -> Task {
        let create_at: DateTime<Utc> = Utc::now();
        Task { text:text, created_at: create_at}
    }
}

pub fn add_task(journal_path: PathBuf, task: Task) -> std::io::Result<()>{
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;
    
    let mut tasks: Vec<Task> = collect_task(&file)?;
    file.seek(SeekFrom::Start(0))?;

    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> std::io::Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(journal_path)?;

    let mut tasks = collect_task(&file)?;

    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    
    tasks.remove(task_position - 1);

    file.seek(SeekFrom::Start(0))?;
    // clear all file
    file.set_len(0)?;

    serde_json::to_writer(file, &tasks)?;
    Ok(())
}


pub fn list_tasks(journal_path: PathBuf) -> std::io::Result<()> {
    let file = OpenOptions::new().read(true).open(journal_path)?;
    let tasks :Vec<Task> = collect_task(&file)?;

    if tasks.is_empty() {
        println!("Task list is empty");
    } else {
        for(index, task) in tasks.iter().enumerate(){
            println!("No.{}: {}", index + 1, task);
        }
    }

    Ok(())
}


fn collect_task(mut file: &File) -> std::io::Result<Vec<Task>>{
    file.seek(SeekFrom::Start(0))?;
    let tasks:Vec<Task> = match serde_json::from_reader(file){
        Ok(tasks) => tasks,
        Err(e)if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    file.seek(SeekFrom::Start(0))?;
    Ok(tasks)
}
