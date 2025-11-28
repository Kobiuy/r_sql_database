use std::collections::HashMap;

use crate::{
    command_history::CommandHistory, custom_error::CustomError, database::AnyDatabase,
    handlers::handle_input_any_db,
};
#[derive(Debug, PartialEq)]
pub enum CurrentScreen {
    Main,

    // Common
    SelectTable,
    SelectField,
    Results,

    // Create Screens
    SelectKeyField,
    InputTableName,
    InputFieldTypePair,

    // Insert
    InputFieldValuePair,

    // Delete
    InputKeyValue,

    // Save_As and Read_From
    InputFilePath,

    // Select
    SelectCondition,
}

#[derive(Debug)]
pub enum CurrentCommand {
    None,
    Create,
    Insert,
    Select,
    Delete,
    SaveAs,
    ReadFrom,
}

pub struct App {
    pub history: CommandHistory,
    pub database: AnyDatabase,

    pub current_input: String,
    pub input_table_name: Option<String>,
    pub selected_type: Option<String>,
    pub generic_hashmap: HashMap<String, String>,
    pub input_field: Option<String>,
    pub input_value: Option<String>,

    pub key_value: Option<String>,

    pub selected_fields: Vec<String>,

    pub selected_condition: Option<String>,
    pub condition_field: Option<String>,
    pub condition_value: Option<String>,

    pub file_path: Option<String>,

    pub current_screen: CurrentScreen,
    pub current_command: CurrentCommand,

    pub field_or_value: FieldType,
    pub current_index: usize,
    pub possibilities: Vec<String>,

    pub result: String,
}
#[derive(PartialEq)]
pub enum FieldType {
    Field,
    Value,
}

impl App {
    pub fn new(database: AnyDatabase, history: CommandHistory) -> Self {
        Self {
            history,
            current_input: String::new(),
            input_table_name: None,
            selected_type: None,
            generic_hashmap: HashMap::new(),
            key_value: None,
            selected_fields: Vec::new(),
            selected_condition: None,
            condition_value: None,
            condition_field: None,
            file_path: None,
            current_screen: CurrentScreen::Main,
            current_command: CurrentCommand::None,
            current_index: 0,
            database,
            field_or_value: FieldType::Field,
            input_field: None,
            input_value: None,
            possibilities: Vec::new(),
            result: String::new(),
        }
    }

    pub fn reset(&mut self) {
        self.current_input.clear();
        self.input_table_name = None;
        self.selected_type = None;
        self.generic_hashmap.clear();
        self.generic_hashmap.clear();
        self.key_value = None;
        self.selected_fields.clear();
        self.selected_condition = None;
        self.condition_value = None;
        self.condition_field = None;
        self.file_path = None;
        self.current_screen = CurrentScreen::Main;
        self.current_command = CurrentCommand::None;
    }
    pub fn go_to_next_page(&mut self) -> Result<(), CustomError> {
        match (&self.current_command, &self.current_screen) {
            (CurrentCommand::Create, CurrentScreen::InputTableName) => {
                self.current_screen = CurrentScreen::InputFieldTypePair;
            }
            (CurrentCommand::Create, CurrentScreen::InputFieldTypePair) => {
                self.current_screen = CurrentScreen::SelectKeyField;
            }
            (CurrentCommand::Create, CurrentScreen::SelectKeyField) => {
                let fields = self
                    .generic_hashmap
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join(", ");

                let table = match &self.input_table_name {
                    Some(v) => v.as_str(),
                    None => "",
                };

                let key = match &self.key_value {
                    Some(v) => v.as_str(),
                    None => "",
                };

                let input = format!("CREATE {table} KEY {key} FIELDS {fields}");

                match handle_input_any_db(input, &mut self.database, &mut self.history) {
                    Ok(res) => self.result = res,
                    Err(e) => self.result = e.to_string(),
                }
                self.current_screen = CurrentScreen::Results;
            }
            (CurrentCommand::Insert, CurrentScreen::SelectTable) => {
                self.current_screen = CurrentScreen::InputFieldValuePair;
            }
            (CurrentCommand::Insert, CurrentScreen::InputFieldValuePair) => {
                let fields = self
                    .generic_hashmap
                    .iter()
                    .map(|(k, v)| format!("{} = {}", k, v))
                    .collect::<Vec<_>>()
                    .join(", ");

                let table = match &self.input_table_name {
                    Some(v) => v.as_str(),
                    None => "",
                };

                let input = format!("INSERT {fields} INTO {table}");

                match handle_input_any_db(input, &mut self.database, &mut self.history) {
                    Ok(res) => self.result = res,
                    Err(e) => self.result = e.to_string(),
                }
                self.current_screen = CurrentScreen::Results;
            }
            (CurrentCommand::Select, CurrentScreen::SelectTable) => {
                let selected_table_name = match &self.input_table_name {
                    Some(v) => v,
                    None => return Err(CustomError::TableNameNotPresent()),
                };
                self.possibilities = self.database.get_fields(selected_table_name)?;
                self.current_screen = CurrentScreen::SelectField;
            }
            (CurrentCommand::Select, CurrentScreen::SelectField) => {
                self.current_screen = CurrentScreen::SelectCondition;
            }
            (CurrentCommand::Select, CurrentScreen::SelectCondition) => {
                let fields = self.selected_fields.join(", ");

                let table = match &self.input_table_name {
                    Some(v) => v.as_str(),
                    None => "",
                };
                let c0 = match &self.condition_field {
                    Some(f) => f,
                    None => "",
                };

                let c1 = match &self.selected_condition {
                    Some(op) => op.to_string(),
                    None => "".to_string(),
                };

                let c2 = match &self.condition_value {
                    Some(v) => v,
                    None => "",
                };

                let mut input = format!("SELECT {} FROM {}", fields, table);

                if !c0.is_empty() && !c1.is_empty() && !c2.is_empty() {
                    input = format!("{} WHERE {} {} {}", input, c0, c1, c2);
                }

                match handle_input_any_db(input, &mut self.database, &mut self.history) {
                    Ok(res) => self.result = res,
                    Err(e) => self.result = e.to_string(),
                }
                self.current_screen = CurrentScreen::Results;
            }
            (CurrentCommand::Delete, CurrentScreen::SelectTable) => {
                self.current_screen = CurrentScreen::InputKeyValue;
            }
            (CurrentCommand::Delete, CurrentScreen::InputKeyValue) => {
                let table = match &self.input_table_name {
                    Some(v) => v.as_str(),
                    None => "",
                };
                let key = match &self.key_value {
                    Some(f) => f,
                    None => "",
                };
                let input = format!("DELETE {} FROM {}", key, table);

                match handle_input_any_db(input, &mut self.database, &mut self.history) {
                    Ok(res) => self.result = res,
                    Err(e) => self.result = e.to_string(),
                }
                self.current_screen = CurrentScreen::Results;
            }
            (CurrentCommand::SaveAs, CurrentScreen::InputFilePath) => {
                let path = match &self.file_path {
                    Some(v) => v.as_str(),
                    None => "",
                };
                let input = format!("SAVE_AS {} ", path);

                match handle_input_any_db(input, &mut self.database, &mut self.history) {
                    Ok(res) => self.result = res,
                    Err(e) => self.result = e.to_string(),
                }
                self.current_screen = CurrentScreen::Results;
            }
            (CurrentCommand::ReadFrom, CurrentScreen::InputFilePath) => {
                let path = match &self.file_path {
                    Some(v) => v.as_str(),
                    None => "",
                };
                let input = format!("READ_FROM {} ", path);

                match handle_input_any_db(input, &mut self.database, &mut self.history) {
                    Ok(res) => self.result = res,
                    Err(e) => self.result = e.to_string(),
                }
                self.current_screen = CurrentScreen::Results;
            }
            (_, CurrentScreen::Results) => {
                self.reset();
                self.current_screen = CurrentScreen::Main;
            }
            (a, b) => {
                return Err(CustomError::ScreenInvalidForCommand(
                    format!("{:?}", a),
                    format!("{:?}", b),
                ));
            }
        }
        self.current_input.clear();
        self.current_index = 0;
        Ok(())
    }
    pub fn select_current_command(&mut self) {
        self.current_command = match self.current_index {
            0 => CurrentCommand::Create,
            1 => CurrentCommand::Insert,
            2 => CurrentCommand::Select,
            3 => CurrentCommand::Delete,
            4 => CurrentCommand::SaveAs,
            5 => CurrentCommand::ReadFrom,
            _ => CurrentCommand::None,
        };
        self.current_index = 0;
        // Set the first screen of that command
        self.current_screen = match self.current_command {
            CurrentCommand::Create => CurrentScreen::InputTableName,
            CurrentCommand::Insert => CurrentScreen::SelectTable,
            CurrentCommand::Select => CurrentScreen::SelectTable,
            CurrentCommand::Delete => CurrentScreen::SelectTable,
            CurrentCommand::SaveAs => CurrentScreen::InputFilePath,
            CurrentCommand::ReadFrom => CurrentScreen::InputFilePath,
            _ => CurrentScreen::Main,
        };
    }
}
