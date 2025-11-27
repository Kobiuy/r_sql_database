use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::{
    app::{App, CurrentScreen, FieldType},
    condition::Op,
    custom_error::CustomError,
};

pub fn handle_events(app: &mut App) -> Result<bool, CustomError> {
    if event::poll(std::time::Duration::from_millis(100))? {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::PageUp => match app.current_screen {
                    CurrentScreen::InputFieldTypePair => {
                        let field_type =
                            match app.database.get_possible_types().get(app.current_index) {
                                Some(v) => v.to_string(),
                                None => {
                                    return Err(CustomError::ItemWithIndexNotFound(
                                        "Type".to_string(),
                                        app.current_index.to_string(),
                                    ));
                                }
                            };

                        app.generic_hashmap
                            .insert(app.current_input.clone(), field_type);
                        app.current_input.clear();
                    }
                    CurrentScreen::SelectField => {
                        app.selected_fields.push(app.current_input.clone());
                        app.current_input.clear();
                    }
                    CurrentScreen::InputFieldValuePair => {
                        if app.field_or_value == FieldType::Field {
                            app.input_field = Some(app.current_input.clone());
                        } else if app.field_or_value == FieldType::Value {
                            app.input_value = Some(app.current_input.clone());
                        }
                        let field = match &app.input_field {
                            Some(v) => v.clone(),
                            None => {
                                return Err(CustomError::MissingField("field".to_string()));
                            }
                        };

                        let field_type = match &app.input_value {
                            Some(v) => v.clone(),
                            None => {
                                return Err(CustomError::MissingField("type".to_string()));
                            }
                        };
                        app.input_field = None;
                        app.input_value = None;
                        app.current_input.clear();
                        app.generic_hashmap.insert(field, field_type);
                    }
                    _ => {}
                },
                KeyCode::Up => match app.current_screen {
                    CurrentScreen::Main
                    | CurrentScreen::SelectKeyField
                    | CurrentScreen::SelectTable
                    | CurrentScreen::SelectCondition
                    | CurrentScreen::InputFieldTypePair => {
                        if app.current_index > 0 {
                            app.current_index -= 1;
                        }
                    }
                    _ => {}
                },
                KeyCode::Down => match app.current_screen {
                    CurrentScreen::Main => {
                        if app.current_index < 5 {
                            app.current_index += 1;
                        }
                    }
                    CurrentScreen::SelectKeyField => {
                        if app.current_index < app.generic_hashmap.len() - 1 {
                            app.current_index += 1;
                        }
                    }
                    CurrentScreen::SelectTable => {
                        if app.current_index < app.database.get_table_names().len() - 1 {
                            app.current_index += 1;
                        }
                    }
                    CurrentScreen::InputFieldTypePair => {
                        if app.current_index < app.database.get_possible_types().len() - 1 {
                            app.current_index += 1;
                        }
                    }
                    CurrentScreen::SelectCondition => {
                        if app.current_index < Op::get_options().len() - 1 {
                            app.current_index += 1;
                        }
                    }
                    _ => {}
                },
                KeyCode::Right => match app.current_screen {
                    CurrentScreen::InputFieldValuePair => {
                        if app.field_or_value == FieldType::Field {
                            app.field_or_value = FieldType::Value;
                            app.input_field = Some(app.current_input.clone());
                            app.current_input = match &app.input_value {
                                Some(v) => v.clone(),
                                None => "".to_string(),
                            }
                        }
                    }
                    CurrentScreen::SelectCondition => {
                        if app.field_or_value == FieldType::Field {
                            app.field_or_value = FieldType::Value;
                            app.condition_field = Some(app.current_input.clone());
                            app.current_input = match &app.condition_value {
                                Some(v) => v.clone(),
                                None => "".to_string(),
                            }
                        }
                    }

                    _ => {}
                },

                KeyCode::Left => match app.current_screen {
                    CurrentScreen::InputFieldValuePair => {
                        if app.field_or_value == FieldType::Value {
                            app.field_or_value = FieldType::Field;
                            app.input_value = Some(app.current_input.clone());
                            app.current_input = match &app.input_field {
                                Some(v) => v.clone(),
                                None => "".to_string(),
                            }
                        }
                    }
                    CurrentScreen::SelectCondition => {
                        if app.field_or_value == FieldType::Value {
                            app.field_or_value = FieldType::Field;
                            app.condition_value = Some(app.current_input.clone());
                            app.current_input = match &app.condition_field {
                                Some(v) => v.clone(),
                                None => "".to_string(),
                            }
                        }
                    }
                    _ => {}
                },

                KeyCode::Char(c) => match app.current_screen {
                    CurrentScreen::InputTableName
                    | CurrentScreen::InputFieldTypePair
                    | CurrentScreen::InputFieldValuePair
                    | CurrentScreen::SelectCondition
                    | CurrentScreen::InputFilePath
                    | CurrentScreen::InputKeyValue
                    | CurrentScreen::SelectField => app.current_input.push(c),
                    _ => {}
                },
                KeyCode::Backspace => match app.current_screen {
                    CurrentScreen::InputTableName
                    | CurrentScreen::SelectField
                    | CurrentScreen::InputFieldTypePair
                    | CurrentScreen::SelectCondition
                    | CurrentScreen::InputFilePath
                    | CurrentScreen::InputKeyValue
                    | CurrentScreen::InputFieldValuePair => {
                        app.current_input.pop();
                    }
                    _ => {}
                },
                KeyCode::Enter => match app.current_screen {
                    CurrentScreen::InputTableName => {
                        app.input_table_name = Some(app.current_input.clone());
                        app.go_to_next_page()?;
                    }
                    CurrentScreen::InputKeyValue => {
                        app.key_value = Some(app.current_input.clone());
                        app.go_to_next_page()?;
                    }
                    CurrentScreen::SelectKeyField => {
                        if let Some(selected_key) = app.key_possibilities.get(app.current_index) {
                            app.key_value = Some(selected_key.clone());
                        } else {
                            return Err(CustomError::InvalidIndex(app.current_index));
                        }
                        app.go_to_next_page()?;
                    }
                    CurrentScreen::SelectTable => {
                        if let Some(selected_table) =
                            app.database.get_table_names().get(app.current_index)
                        {
                            app.input_table_name = Some(selected_table.clone());
                        } else {
                            return Err(CustomError::InvalidIndex(app.current_index));
                        }
                        app.go_to_next_page()?;
                    }
                    CurrentScreen::InputFieldTypePair => {
                        app.key_value = Some(app.current_input.clone());
                        app.key_possibilities = app
                            .generic_hashmap
                            .iter()
                            .filter(|(_, v)| **v == app.database.get_key_type())
                            .map(|(k, _)| k.clone())
                            .collect();
                        app.go_to_next_page()?;
                    }
                    CurrentScreen::InputFieldValuePair => {
                        app.go_to_next_page()?;
                    }
                    CurrentScreen::SelectField => {
                        app.go_to_next_page()?;
                    }
                    CurrentScreen::Results => {
                        app.go_to_next_page()?;
                    }
                    CurrentScreen::SelectCondition => {
                        match app.field_or_value {
                            FieldType::Field => {
                                app.condition_field = Some(app.current_input.clone())
                            }
                            FieldType::Value => {
                                app.condition_value = Some(app.current_input.clone())
                            }
                        }
                        app.selected_condition = Op::get_options().get(app.current_index).cloned();

                        app.go_to_next_page()?;
                    }
                    CurrentScreen::InputFilePath => {
                        app.file_path = Some(app.current_input.clone());
                        app.go_to_next_page()?;
                    }
                    CurrentScreen::Main => app.select_current_command(),
                },
                KeyCode::Esc => match app.current_screen {
                    CurrentScreen::Main => return Ok(true),
                    _ => app.reset(),
                },
                _ => {}
            },
            _ => {}
        }
    }
    Ok(false)
}
