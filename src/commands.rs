use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, Write};

use crate::command_history::CommandHistory;
use crate::custom_error::CustomError;
use crate::database::DatabaseKey;
use crate::database::{self, Table};
use crate::handlers::handle_input;
use crate::parsers::{self, parse_fields_list};
use database::Database;
use parsers::parse_fields;

pub trait Command {
    fn execute(&mut self) -> Result<String, CustomError>;
}

pub trait Serialize {
    fn serialize(&mut self) -> String;
}
pub struct Create<'a, K: DatabaseKey> {
    database: &'a mut Database<K>,
    name: String,
    key_field: String,
    fields: HashMap<String, String>,
}

pub struct Insert<'a, K: DatabaseKey> {
    table: &'a mut Table<K>,
    record_string: String,
}
pub struct Delete<'a, K: DatabaseKey> {
    table: &'a mut Table<K>,
    key_string: String,
}
pub struct Select<'a, K: DatabaseKey> {
    table: &'a mut Table<K>,
    fields: Vec<String>,
    cond_string: Option<String>,
}

pub struct SaveAs<'a> {
    file_path: String,
    history: &'a mut CommandHistory,
}

pub struct ReadFrom<'a, K: DatabaseKey> {
    database: &'a mut Database<K>,
    file_path: String,
    history: &'a mut CommandHistory,
}

impl<'a, K: DatabaseKey> Create<'a, K> {
    pub fn new(database: &'a mut Database<K>, input: &str) -> Result<Self, CustomError> {
        let key_pos = input
            .find("KEY")
            .ok_or_else(|| CustomError::MissingKeyword("KEY".to_string()))?;
        let table_name = input["CREATE".len()..key_pos].trim();
        if table_name.is_empty() {
            return Err(CustomError::MissingField("Table name".to_string()));
        }

        let after_key = &input[key_pos + "KEY".len()..];

        let fields_pos = after_key
            .find("FIELDS")
            .ok_or_else(|| CustomError::MissingKeyword("FIELDS".to_string()))?;
        let key_field = after_key[..fields_pos].trim();
        if key_field.is_empty() {
            return Err(CustomError::MissingField("Key field".to_string()));
        }

        let fields_str = after_key[fields_pos + "FIELDS".len()..].trim();
        let fields = parse_fields(fields_str)?;

        Ok(Self {
            database,
            name: table_name.into(),
            key_field: key_field.into(),
            fields,
        })
    }
}

impl<'a, K: DatabaseKey> Insert<'a, K> {
    pub fn new(database: &'a mut Database<K>, input: &str) -> Result<Self, CustomError> {
        let into_pos = input
            .find("INTO")
            .ok_or_else(|| CustomError::MissingKeyword("INTO".to_string()))?;

        let record_string = input["INSERT".len()..into_pos].trim().to_string();
        if record_string.is_empty() {
            return Err(CustomError::MissingField("Record data".to_string()));
        }

        let after_into = &input[into_pos + "INTO".len()..];
        let table_name = after_into.trim();
        if table_name.is_empty() {
            return Err(CustomError::MissingField("Table name".to_string()));
        }

        let table = database.get_table_mut(table_name)?;

        Ok(Self {
            table,
            record_string,
        })
    }
}

impl<'a, K: DatabaseKey> Delete<'a, K> {
    pub fn new(database: &'a mut Database<K>, input: &str) -> Result<Self, CustomError> {
        input
            .find("DELETE")
            .ok_or_else(|| CustomError::MissingKeyword("DELETE".to_string()))?;

        let from_pos = input
            .find("FROM")
            .ok_or_else(|| CustomError::MissingKeyword("FROM".to_string()))?;

        let table_name = &input["FROM".len() + from_pos..].trim().to_string();
        let key_string = input["DELETE".len()..from_pos].trim().to_string();
        let table = database.get_table_mut(table_name)?;

        Ok(Self { table, key_string })
    }
}
impl<'a, K: DatabaseKey> Select<'a, K> {
    pub fn new(database: &'a mut Database<K>, input: &str) -> Result<Self, CustomError> {
        input
            .find("SELECT")
            .ok_or_else(|| CustomError::MissingKeyword("SELECT".to_string()))?;

        let from_pos = input
            .find("FROM")
            .ok_or_else(|| CustomError::MissingKeyword("FROM".to_string()))?;

        let where_pos = input.find("WHERE");

        let fields_str = input["SELECT".len()..from_pos].trim();
        let fields = parse_fields_list(fields_str);

        let (table_name, cond_string) = if let Some(pos) = where_pos {
            let name = input[from_pos + "FROM".len()..pos].trim();
            let cond_string = input[pos + "WHERE".len()..].trim().to_string();
            (name, Some(cond_string))
        } else {
            let name = input[from_pos + "FROM".len()..].trim();
            (name, None)
        };
        let table = database.get_table_mut(table_name)?;
        Ok(Self {
            table,
            fields,
            cond_string,
        })
    }
}
impl<'a> SaveAs<'a> {
    pub fn new(input: &str, history: &'a mut CommandHistory) -> Result<Self, CustomError> {
        input
            .find("SAVE_AS")
            .ok_or_else(|| CustomError::MissingKeyword("SAVE_AS".to_string()))?;

        let file_path = input["SAVE_AS".len()..].trim().to_string();

        Ok(Self { file_path, history })
    }
}

impl<'a, K: DatabaseKey> ReadFrom<'a, K> {
    pub fn new(
        input: &str,
        history: &'a mut CommandHistory,
        database: &'a mut Database<K>,
    ) -> Result<Self, CustomError> {
        input
            .find("READ_FROM")
            .ok_or_else(|| CustomError::MissingKeyword("READ_FROM".to_string()))?;

        let file_path = input["READ_FROM".len()..].trim().to_string();

        Ok(Self {
            file_path,
            history,
            database,
        })
    }
}

impl<'a, K: DatabaseKey> Command for Create<'a, K> {
    fn execute(&mut self) -> Result<String, CustomError> {
        match self.database.create_table(
            self.name.clone(),
            self.key_field.clone(),
            self.fields.clone(),
        ) {
            Ok(_) => Ok("Table created succesfully".to_string()),
            Err(e) => Err(e),
        }
    }
}

impl<'a, K: DatabaseKey> Command for Insert<'a, K> {
    fn execute(&mut self) -> Result<String, CustomError> {
        let record = self.table.parse_record(&self.record_string)?;

        match self.table.add_record(record) {
            Ok(_) => Ok("Data inserted succesfully".to_string()),
            Err(e) => Err(e),
        }
    }
}
impl<'a, K: DatabaseKey> Command for Delete<'a, K> {
    fn execute(&mut self) -> Result<String, CustomError> {
        match self.table.remove_record(&self.key_string) {
            Ok(_) => Ok("Data deleted succesfully".to_string()),
            Err(e) => Err(e),
        }
    }
}
impl<'a, K: DatabaseKey> Command for Select<'a, K> {
    fn execute(&mut self) -> Result<String, CustomError> {
        match self.table.select_records(&self.fields, &self.cond_string) {
            Ok(v) => {
                let result = v
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" | ");
                Ok(result)
            }
            Err(e) => Err(e),
        }
    }
}

impl<'a> Command for SaveAs<'a> {
    fn execute(&mut self) -> Result<String, CustomError> {
        let mut file = File::create(&self.file_path).map_err(CustomError::IoError)?;
        for entry in self.history.list() {
            match writeln!(file, "{}", entry) {
                Ok(_) => {}
                Err(e) => return Err(CustomError::IoError(e)),
            };
        }
        Ok("Data saved succesfully".to_string())
    }
}

impl<'a, K: DatabaseKey> Command for ReadFrom<'a, K> {
    fn execute(&mut self) -> Result<String, CustomError> {
        let file = File::open(&self.file_path).map_err(CustomError::IoError)?;

        let reader = std::io::BufReader::new(file);

        for line in reader.lines() {
            let text = line.map_err(CustomError::IoError)?;
            handle_input(text, self.database, self.history)?;
        }

        Ok("Data read succesfully".to_string())
    }
}

impl<'a, K: DatabaseKey> Serialize for Create<'a, K> {
    fn serialize(&mut self) -> String {
        format!(
            "CREATE {} KEY {} FIELDS {} ",
            self.name,
            self.key_field,
            self.fields
                .iter()
                .map(|(name, typ)| format!("{}: {}", name, typ))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl<'a, K: DatabaseKey> Serialize for Insert<'a, K> {
    fn serialize(&mut self) -> String {
        format!(
            "INSERT {} INTO {}",
            self.record_string, self.table.table_name
        )
    }
}
impl<'a, K: DatabaseKey> Serialize for Delete<'a, K> {
    fn serialize(&mut self) -> String {
        format!("DELETE {} FROM {}", self.key_string, self.table.table_name)
    }
}
impl<'a, K: DatabaseKey> Serialize for Select<'a, K> {
    fn serialize(&mut self) -> String {
        match &self.cond_string {
            Some(cond_string) => format!(
                "SELECT {} FROM {} WHERE {}",
                self.fields.join(", "),
                self.table.table_name,
                cond_string
            ),
            None => format!(
                "SELECT {} FROM {}",
                self.fields.join(", "),
                self.table.table_name,
            ),
        }
    }
}
