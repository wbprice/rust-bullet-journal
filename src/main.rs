use structopt::StructOpt;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use termion::event::Key;
use termion::input::TermRead;

mod controllers;
mod models;
mod services;
mod views;

use crate::{
    controllers::{Command, EntryOpts, Opt},
    models::{Entry, EntryVariants},
    services::{Journalable, LocalDiskJournal},
    views::Application,
};

fn main() {
    let opt = Opt::from_args();

    let mut journal = LocalDiskJournal::new(None);

    // Handle input!
    if let Some(command) = opt.command {
        match command {
            Command::Add { new_entry } => match new_entry {
                EntryOpts::Event { text } => journal.append(Entry::new(EntryVariants::Event, text)),
                EntryOpts::Note { text } => journal.append(Entry::new(EntryVariants::Note, text)),
                EntryOpts::Task { text } => journal.append(Entry::new(EntryVariants::Task, text)),
            },
            Command::Emph { index } => {
                journal.toggle_importance(index);
            }
            Command::Complete { index } => {
                journal.toggle_completion(index);
            }
            Command::Remove { index } => journal.remove(index),
        }
    }

    let application = Application {
        stdout: stdout().into_raw_mode().unwrap(),
        entries: journal.list(),
    };

    application.render();

    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    writeln!(stdout, "Hey there.").unwrap();

    for c in stdin.keys() {
        
        match c.unwrap() {
            Key::Char('q') => break,
            _ => {}
        }

        stdout.flush().unwrap();
    }
}
