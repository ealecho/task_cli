use clap::{App, Arg};

pub fn build_new_task() -> App<'static, 'static> {
    let new_task_command = App::new("tasks")
    .about("Create a task")
    .aliases(&["task"])
    .arg(
        Arg::new("data")
            .short('d')
            .help("a new task data"),
    );

    new_task_command
}