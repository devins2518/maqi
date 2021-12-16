use log::LevelFilter;
use simplelog::{Config, WriteLogger};
use std::{error::Error, fs::File};

mod application;
mod client;
mod imap;
mod terminal;
mod ui;
mod utils;

const APP_NAME: &str = "Maqi";

fn main() -> Result<(), Box<dyn Error>> {
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create("/tmp/maqi.log").unwrap(),
    )
    .unwrap();

    let mut app = application::Application::new(APP_NAME);
    if let Err(e) = app.run() {
        eprintln!("Error received when running maqi!\n{}", e);
        std::process::exit(1);
    }

    Ok(())
}
