mod frontend;
mod backend;

use std::env::current_dir;

use iced::Task;

use crate::frontend::application::Application;

fn main() -> Result<(), iced::Error> {

    let path = match current_dir() {
        Ok(path) => path,
        Err(_) => {
            eprintln!("[ERROR] Not a valid CWD.");
            return Ok(());
        }
    };

    iced::application("Oxidrive", Application::update, Application::view).run_with(
        || (Application::new(path.clone()), Task::done(frontend::message::Message::ReadDirectory(path)))
    )
}
