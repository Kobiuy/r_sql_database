use crate::{
    condition::{Condition, Op},
    custom_error::CustomError,
};
use std::collections::HashMap;
pub fn parse_fields(fields_str: &str) -> Result<HashMap<String, String>, CustomError> {
    let mut fields: HashMap<String, String> = HashMap::new();

    for part in fields_str.split(',') {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        let mut kv = trimmed.split(':');
        if let (Some(name), Some(typ)) = (kv.next(), kv.next()) {
            fields.insert(name.trim().to_string(), typ.trim().to_string());
        } else {
            return Err(CustomError::FieldParseError(trimmed.to_string()));
        }
    }

    Ok(fields)
}
pub fn parse_conditions(
    cond_str: &str,
    fields: &HashMap<String, String>,
) -> Result<Vec<Condition>, CustomError> {
    cond_str
        .split(',')
        .map(|cond| {
            let parts: Vec<&str> = cond.split_whitespace().collect();
            if parts.len() != 3 {
                return Err(CustomError::ConditionParseError(
                    "Invalid condition format".to_string(),
                ));
            }

            let field_name = parts[0];
            let op_str = parts[1];
            let value_str = parts[2];

            let op = Op::as_str(op_str)
                .ok_or_else(|| CustomError::ConditionParseError("Unknown operator".to_string()))?;

            let field_type = fields
                .get(field_name)
                .ok_or_else(|| CustomError::ConditionParseError("Field not found".to_string()))?;

            Condition::new(field_name, op, value_str, field_type).map_err(|_| {
                CustomError::ConditionParseError("Failed to parse value for condition".to_string())
            })
        })
        .collect::<Result<Vec<Condition>, CustomError>>()
}
pub fn parse_fields_list(fields_str: &str) -> Vec<String> {
    fields_str
        .split(',')
        .map(|a| a.trim().to_string())
        .collect()
}
