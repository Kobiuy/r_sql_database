use std::{
    any::Any,
    collections::{BTreeMap, HashMap, btree_map::Entry},
    fmt::{self, Display, Formatter},
};

use crate::{custom_error::CustomError, parsers::parse_conditions};

pub trait DatabaseKey: Ord {
    fn is_equal_to(&self, other: &Self) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn from_value(v: &Value) -> Result<Self, CustomError>
    where
        Self: Sized;
    fn to_string_2(&self) -> String;
    fn key_type_name() -> &'static str;
}

#[derive(PartialEq, Clone)]
pub enum Value {
    Bool(bool),
    String(String),
    Int(i64),
    Float(f64),
}

#[derive(Clone)]
pub struct Record {
    pub values: HashMap<String, Value>,
}
pub struct Table<K: DatabaseKey> {
    pub table_name: String,
    pub key_field: String,
    pub fields: HashMap<String, String>,
    pub records: BTreeMap<K, Record>,
}
pub struct Database<K: DatabaseKey> {
    pub tables: HashMap<String, Table<K>>,
}

pub enum AnyDatabase {
    StringDatabase(Database<String>),
    IntDatabase(Database<i64>),
}

impl AnyDatabase {
    pub fn get_table_names(&self) -> Vec<String> {
        match self {
            AnyDatabase::StringDatabase(database) => database.tables.keys().cloned().collect(),
            AnyDatabase::IntDatabase(database) => database.tables.keys().cloned().collect(),
        }
    }
    pub fn get_possible_types(&self) -> Vec<&str> {
        vec!["Int", "String", "Float", "Bool"]
    }
    pub fn get_key_type(&self) -> String {
        match self {
            AnyDatabase::StringDatabase(_) => "String".to_string(),
            AnyDatabase::IntDatabase(_) => "Int".to_string(),
        }
    }
}

impl Value {
    pub fn from_string(s: &str, type_hint: &str) -> Option<Self> {
        let t = type_hint.trim().to_ascii_uppercase();

        match t.as_str() {
            "INT" => s.parse::<i64>().ok().map(Value::Int),
            "FLOAT" => s.parse::<f64>().ok().map(Value::Float),
            "STRING" => Some(Value::String(s.to_string())),
            "BOOL" => s.parse::<bool>().ok().map(Value::Bool),
            _ => None,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Value::Bool(b) => b.to_string(),
            Value::String(s) => s.clone(),
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
        }
    }
}
impl Record {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn with_values(values: HashMap<String, Value>) -> Self {
        Self { values }
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = self
            .values
            .iter()
            .map(|(key, value)| format!("{}={}", key, value.as_string()))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", s)
    }
}
impl<K: DatabaseKey> Database<K> {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }
    pub fn create_table(
        &mut self,
        name: String,
        key_field: String,
        fields: HashMap<String, String>,
    ) -> Result<(), CustomError> {
        if self.tables.contains_key(&name) {
            return Err(CustomError::TableAlreadyExists(name.to_string()));
        }

        let key_type = match fields.get(&key_field) {
            Some(t) => t.trim().to_uppercase(),
            None => return Err(CustomError::UnknownField(key_field.to_string())),
        };

        if key_type != K::key_type_name() {
            return Err(CustomError::WrongKeyType());
        }

        let table = Table::new(name.clone(), key_field, fields);
        self.tables.insert(name, table);
        Ok(())
    }
    pub fn get_table_mut(&mut self, table_name: &str) -> Result<&mut Table<K>, CustomError> {
        self.tables
            .get_mut(table_name)
            .ok_or_else(|| CustomError::TableNotFound(table_name.to_string()))
    }
}

impl DatabaseKey for String {
    fn is_equal_to(&self, other: &Self) -> bool {
        self == other
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn from_value(v: &Value) -> Result<Self, CustomError> {
        match v {
            Value::String(s) => Ok(s.clone()),
            _ => Err(CustomError::ValueParseError(v.as_string())),
        }
    }

    fn to_string_2(&self) -> String {
        self.as_str().to_string()
    }
    fn key_type_name() -> &'static str {
        "STRING"
    }
}
impl DatabaseKey for i64 {
    fn is_equal_to(&self, other: &Self) -> bool {
        self == other
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn from_value(v: &Value) -> Result<Self, CustomError> {
        match v {
            Value::Int(i) => Ok(*i),
            _ => Err(CustomError::ValueParseError(v.as_string())),
        }
    }
    fn to_string_2(&self) -> String {
        self.to_string()
    }
    fn key_type_name() -> &'static str {
        "INT"
    }
}
impl<K: DatabaseKey> Table<K> {
    fn new(table_name: String, key_field: String, fields: HashMap<String, String>) -> Self {
        Self {
            table_name,
            key_field,
            fields,
            records: BTreeMap::new(),
        }
    }

    pub fn add_record(&mut self, record: Record) -> Result<(), CustomError> {
        let key_value = match record.values.get(&self.key_field) {
            Some(k) => k,
            None => {
                return Err(CustomError::MissingField(self.key_field.to_string()));
            }
        };

        let k_key: K = K::from_value(key_value)?;

        match self.records.entry(k_key) {
            Entry::Vacant(entry) => {
                entry.insert(record);
                Ok(())
            }
            Entry::Occupied(_) => Err(CustomError::RecordAlreadyExists(record.to_string())),
        }
    }

    pub fn remove_record(&mut self, key_str: &str) -> Result<(), CustomError> {
        let key_type = self
            .fields
            .get(&self.key_field)
            .ok_or_else(|| CustomError::InvalidKey(self.key_field.to_string()))?;

        let key_value = Value::from_string(key_str, key_type)
            .ok_or_else(|| CustomError::InvalidKey(key_str.to_string()))?;
        let key = K::from_value(&key_value)?;
        match self.records.remove(&key) {
            Some(_) => Ok(()),
            None => Err(CustomError::InvalidKey(key_str.to_string())),
        }
    }
    pub fn parse_record(&mut self, record_str: &str) -> Result<Record, CustomError> {
        let mut record = Record::new();

        for part in record_str.split(',') {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                continue;
            }

            let mut kv = trimmed.split('=');
            let (Some(name), Some(value)) = (kv.next(), kv.next()) else {
                return Err(CustomError::FieldParseError(trimmed.to_string()));
            };

            let field_type = self
                .fields
                .get(name.trim())
                .ok_or_else(|| CustomError::FieldParseError(name.to_string()))?;

            let value = Value::from_string(value.trim(), field_type)
                .ok_or_else(|| CustomError::FieldParseError(name.to_string()))?;

            record.values.insert(name.trim().to_string(), value);
        }

        Ok(record)
    }
    pub fn select_records(
        &mut self,
        fields: Vec<String>,
        conditions_str: Option<String>,
    ) -> Result<Vec<Record>, CustomError> {
        let cond = match conditions_str {
            Some(s) => Some(parse_conditions(&s, &self.fields)?),
            None => None,
        };

        let result = self
            .records
            .values()
            .filter(|r| match &cond {
                Some(cnd) => cnd.iter().any(|c| {
                    if let Some(val) = r.values.get(&c.field_name) {
                        c.op.cmp(val, &c.value)
                    } else {
                        false
                    }
                }),
                None => true,
            })
            .map(|r| {
                let mut r2 = r.clone();
                r2.values = r
                    .values
                    .iter()
                    .filter(|(k, _v)| fields.contains(&k.to_string_2()))
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                r2
            })
            .collect::<Vec<_>>();

        Ok(result)
    }
}
