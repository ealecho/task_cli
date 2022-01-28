
use clap;
use std::{process, fmt::Result};

use infrastructure::handlers::tasks::TaskHandler;
use interface::controllers::tasks::TaskController;
use usecases::interactors::tasks::CreateTaskInteractor;

mod cmd;
mod app;

use crate::app::App;

fn run() -> Result<bool, String> {
    let app = App::new()?;
    println!("Debug: {:?},", app.matches);

    let i = CreateTaskInteractor::new();
    let c = TaskController::new(i);
    let h = TaskHandler::new(c);

    match app.matches.subcommand() {
        ("new", Some(new_matches)) => match new_matches.subcommand() {
            ("tasks", Some(task_matches)) => {
                h.create(task_matches);
                Ok(true)
            },
            ("", None) => Err(String::from("too bad")),
            _ => Ok(true),
        },
        ("", None) => {
            println!("No subcommand was used");
            Ok(true)
        }
        _ => Ok(true),
    }
}
fn main() {
    let result = run();

    match result {
        Ok(false) => {
            process::exit(1)
        },
        Ok(true) => {
            process::exit(0)
        },
        Err(_error) => process::exit(1),
    }
}
