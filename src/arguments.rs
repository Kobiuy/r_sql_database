use std::io::{self, Write};

use clap::Parser;

use crate::{
    app::App,
    command_history::CommandHistory,
    database::{AnyDatabase, Database},
    handlers::handle_input_any_db,
    ui::run,
};

#[derive(Parser)]
pub struct Args {
    #[clap(value_enum,default_value_t = KeyType::String)]
    pub key_type: KeyType,
    #[clap(value_enum, default_value_t = UiType::Graphic)]
    pub ui_type: UiType,
}

#[derive(clap::ValueEnum, Clone)]
pub enum KeyType {
    String,
    Int,
}

#[derive(clap::ValueEnum, Clone)]
pub enum UiType {
    Command,
    Graphic,
}

impl KeyType {
    pub fn to_database(&self) -> AnyDatabase {
        match self {
            KeyType::String => AnyDatabase::StringDatabase(Database::<String>::new()),
            KeyType::Int => AnyDatabase::IntDatabase(Database::<i64>::new()),
        }
    }
}

pub fn run_command_line(key_type: KeyType) -> io::Result<()> {
    let mut db = key_type.to_database();
    let mut input = String::new();
    let mut history = CommandHistory::new();

    loop {
        print!("> ");
        io::stdout().flush()?;

        input.clear();
        let bytes = io::stdin().read_line(&mut input)?;
        if bytes == 0 {
            println!("EOF received, exiting.");
            break;
        }
        let trimmed = input.trim();
        if trimmed.is_empty() {
            println!("Empty line -> exiting.");
            break;
        }
        match handle_input_any_db(String::from(input.trim()), &mut db, &mut history) {
            Ok(r) => println!("{}", r),
            Err(e) => println!("{}", e),
        }
    }
    Ok(())
}
pub fn run_graphic_interface(key_type: KeyType) -> io::Result<()> {
    let db = key_type.to_database();
    let history = CommandHistory::new();

    let mut terminal = ratatui::init();
    let mut app = App::new(db, history);
    let result = run(&mut terminal, &mut app);
    ratatui::restore();
    result
}
