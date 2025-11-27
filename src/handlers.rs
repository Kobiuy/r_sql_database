use crate::{
    command_history::CommandHistory,
    commands::{Command, Create, Delete, Insert, ReadFrom, SaveAs, Select, Serialize},
    custom_error::CustomError,
    database::{AnyDatabase, Database, DatabaseKey},
};

pub fn handle_input_any_db(
    input: String,
    database: &mut AnyDatabase,
    history: &mut CommandHistory,
) -> Result<String, CustomError> {
    match database {
        AnyDatabase::StringDatabase(db) => handle_input(input, db, history),
        AnyDatabase::IntDatabase(db) => handle_input(input, db, history),
    }
}
pub fn handle_input(
    input: String,
    database: &mut Database<impl DatabaseKey>,
    history: &mut CommandHistory,
) -> Result<String, CustomError> {
    let trimmed = input.trim();

    match true {
        _ if trimmed.starts_with("CREATE") => handle_create(trimmed, database, history),
        _ if trimmed.starts_with("INSERT") => handle_insert(trimmed, database, history),
        _ if trimmed.starts_with("DELETE") => handle_delete(trimmed, database, history),
        _ if trimmed.starts_with("SELECT") => handle_select(trimmed, database, history),
        _ if trimmed.starts_with("SAVE_AS") => handle_save_as(trimmed, history),
        _ if trimmed.starts_with("READ_FROM") => handle_read_from(trimmed, database, history),
        _ => Err(CustomError::UnknownCommand(trimmed.to_string())),
    }
}

pub fn handle_create<K: DatabaseKey>(
    input: &str,
    database: &mut Database<K>,
    history: &mut CommandHistory,
) -> Result<String, CustomError> {
    let mut command = Create::new(database, input)?;
    history.push(command.serialize());
    command.execute()
}
pub fn handle_insert<K: DatabaseKey>(
    input: &str,
    database: &mut Database<K>,
    history: &mut CommandHistory,
) -> Result<String, CustomError> {
    let mut command = Insert::new(database, input)?;
    history.push(command.serialize());
    command.execute()
}
pub fn handle_delete<K: DatabaseKey>(
    input: &str,
    database: &mut Database<K>,
    history: &mut CommandHistory,
) -> Result<String, CustomError> {
    let mut command = Delete::new(database, input)?;
    history.push(command.serialize());
    command.execute()
}
pub fn handle_select<K: DatabaseKey>(
    input: &str,
    database: &mut Database<K>,
    history: &mut CommandHistory,
) -> Result<String, CustomError> {
    let mut command = Select::new(database, input)?;
    history.push(command.serialize());

    command.execute()
}
pub fn handle_save_as(input: &str, history: &mut CommandHistory) -> Result<String, CustomError> {
    let mut command = SaveAs::new(input, history)?;

    command.execute()
}

pub fn handle_read_from<K: DatabaseKey>(
    input: &str,
    database: &mut Database<K>,
    history: &mut CommandHistory,
) -> Result<String, CustomError> {
    let mut command = ReadFrom::new(input, history, database)?;

    command.execute()
}
