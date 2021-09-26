mod cli;
mod task;

use anyhow::anyhow;
use cli::{Action::*, CommandLineArgs};
use std::path::PathBuf;
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    // cli::CommandLineArgs::from_args();
    // println!("{:#?}", cli::CommandLineArgs::from_args());
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file.or_else(find_default_journal_file).ok_or(anyhow!("Failed to find journal file."))?;

    match action {
        Add { text } => task::add_task(journal_file, task::Task::new(text)),
        Done { position } => task::complete_task(journal_file, position),
        List => task::list_tasks(journal_file),
    }?;

    Ok(())
}

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}
