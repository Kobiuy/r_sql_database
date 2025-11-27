use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::{
    app::{App, CurrentCommand, CurrentScreen, FieldType},
    condition::Op,
    event_handler::handle_events,
};

pub fn run(terminal: &mut ratatui::DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| {
            ui(frame, app);
        })?;

        match handle_events(app) {
            Ok(v) => {
                if v {
                    break Ok(());
                }
            }
            Err(e) => app.result = e.to_string(),
        }
        {}
    }
}

pub fn ui(frame: &mut Frame, app: &App) {
    match app.current_screen {
        CurrentScreen::Main => {
            let items = [
                "Create",
                "Insert",
                "Select",
                "Delete",
                "Save As",
                "Read From",
            ];

            let list_items: Vec<ListItem> = items
                .iter()
                .enumerate()
                .map(|(i, text)| {
                    if i == app.current_index {
                        ListItem::new(*text).style(Style::default().fg(Color::Yellow))
                    } else {
                        ListItem::new(*text).style(Style::default().fg(Color::White))
                    }
                })
                .collect();

            let list =
                List::new(list_items).block(Block::default().title("Menu").borders(Borders::ALL));

            frame.render_widget(list, frame.area());
        }
        CurrentScreen::SelectTable => {
            let list_items: Vec<ListItem> = app
                .database
                .get_table_names()
                .iter()
                .enumerate()
                .map(|(i, text)| {
                    if i == app.current_index {
                        ListItem::new(text.clone()).style(Style::default().fg(Color::Yellow))
                    } else {
                        ListItem::new(text.clone()).style(Style::default().fg(Color::White))
                    }
                })
                .collect();

            let list = List::new(list_items)
                .block(Block::default().title("Select Table").borders(Borders::ALL));

            frame.render_widget(list, frame.area());
        }
        CurrentScreen::Results => {
            let list = Paragraph::new(app.result.clone());
            frame.render_widget(list, frame.area());
        }
        CurrentScreen::InputKeyValue => {
            let title = match app.current_command {
                CurrentCommand::Delete => "Enter Key Of The Record You Want To Delete",
                CurrentCommand::Insert => "Enter Key Field Name",
                CurrentCommand::Select => "Enter Key Field Name",
                _ => "Enter Key",
            };
            let paragraph = Paragraph::new(app.current_input.clone())
                .block(Block::default().title(title).borders(Borders::ALL))
                .style(Style::default().fg(Color::White));

            frame.render_widget(paragraph, frame.area());
        }
        CurrentScreen::SelectKeyField => {
            let list_items: Vec<ListItem> = app
                .key_possibilities
                .iter()
                .enumerate()
                .map(|(i, text)| {
                    if i == app.current_index {
                        ListItem::new(text.clone()).style(Style::default().fg(Color::Yellow))
                    } else {
                        ListItem::new(text.clone()).style(Style::default().fg(Color::White))
                    }
                })
                .collect();

            let list =
                List::new(list_items).block(Block::default().title("Key").borders(Borders::ALL));

            frame.render_widget(list, frame.area());
        }
        CurrentScreen::InputTableName => {
            let paragraph = Paragraph::new(app.current_input.clone())
                .block(
                    Block::default()
                        .title("Enter Table Name")
                        .borders(Borders::ALL),
                )
                .style(Style::default().fg(Color::White));

            frame.render_widget(paragraph, frame.area());
        }
        CurrentScreen::InputFieldTypePair => {
            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Length(7), Constraint::Min(3)].as_ref())
                .split(frame.area());
            let top_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(vertical_chunks[0]);
            let map_contents = if app.generic_hashmap.is_empty() {
                "".to_string()
            } else {
                app.generic_hashmap
                    .iter()
                    .map(|(k, v)| format!("{k}: {v}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            };

            let map_paragraph = Paragraph::new(map_contents)
                .block(Block::default().title("Fields ").borders(Borders::ALL));

            let field_paragraph = Paragraph::new(app.current_input.clone()).block(
                Block::default()
                    .title("Field")
                    .borders(Borders::ALL)
                    .border_style(Style::default()),
            );

            let list_items: Vec<ListItem> = app
                .database
                .get_possible_types()
                .iter()
                .enumerate()
                .map(|(i, text)| {
                    if i == app.current_index {
                        ListItem::new(*text).style(Style::default().fg(Color::Yellow))
                    } else {
                        ListItem::new(*text).style(Style::default().fg(Color::White))
                    }
                })
                .collect();
            let list = List::new(list_items).block(
                Block::default()
                    .title("Type")
                    .borders(Borders::ALL)
                    .border_style(Style::default()),
            );
            frame.render_widget(field_paragraph, top_chunks[0]);
            frame.render_widget(list, top_chunks[1]);
            frame.render_widget(map_paragraph, vertical_chunks[1]);
        }

        CurrentScreen::InputFieldValuePair => {
            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Length(7), Constraint::Min(3)].as_ref())
                .split(frame.area());
            let top_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(vertical_chunks[0]);
            let map_contents = if app.generic_hashmap.is_empty() {
                "".to_string()
            } else {
                app.generic_hashmap
                    .iter()
                    .map(|(k, v)| format!("{k}: {v}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            };

            let map_paragraph = Paragraph::new(map_contents)
                .block(Block::default().title("Fields ").borders(Borders::ALL));

            let field_paragraph = Paragraph::new(if app.field_or_value == FieldType::Field {
                app.current_input.clone()
            } else {
                match &app.input_field {
                    Some(v) => v.clone(),
                    None => "".to_string(),
                }
            })
            .block(
                Block::default()
                    .title("Field")
                    .borders(Borders::ALL)
                    .border_style(if app.field_or_value == FieldType::Field {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    }),
            );
            let value_paragraph = Paragraph::new(if app.field_or_value == FieldType::Value {
                app.current_input.clone()
            } else {
                match &app.input_value {
                    Some(v) => v.clone(),
                    None => "".to_string(),
                }
            })
            .block(
                Block::default()
                    .title("Value")
                    .borders(Borders::ALL)
                    .border_style(if app.field_or_value == FieldType::Value {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    }),
            );
            frame.render_widget(field_paragraph, top_chunks[0]);
            frame.render_widget(value_paragraph, top_chunks[1]);
            frame.render_widget(map_paragraph, vertical_chunks[1]);
        }
        CurrentScreen::SelectField => {
            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Length(7), Constraint::Min(3)].as_ref())
                .split(frame.area());

            let map_contents = if app.selected_fields.is_empty() {
                "".to_string()
            } else {
                app.selected_fields.join(", ")
            };

            let map_paragraph = Paragraph::new(map_contents)
                .block(Block::default().title("Fields").borders(Borders::ALL));

            let field_paragraph = Paragraph::new(app.current_input.clone()).block(
                Block::default()
                    .title("Field")
                    .borders(Borders::ALL)
                    .border_style(Style::default()),
            );

            frame.render_widget(field_paragraph, vertical_chunks[0]);
            frame.render_widget(map_paragraph, vertical_chunks[1]);
        }
        CurrentScreen::SelectCondition => {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(40),
                        Constraint::Percentage(20),
                        Constraint::Percentage(40),
                    ]
                    .as_ref(),
                )
                .split(frame.area());

            let field_paragraph = Paragraph::new(if app.field_or_value == FieldType::Field {
                app.current_input.clone()
            } else {
                match &app.condition_field {
                    Some(v) => v.clone(),
                    None => "".to_string(),
                }
            })
            .block(
                Block::default()
                    .title("Field")
                    .borders(Borders::ALL)
                    .border_style(if app.field_or_value == FieldType::Field {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    }),
            );
            let list_items: Vec<ListItem> = Op::get_options()
                .iter()
                .enumerate()
                .map(|(i, text)| {
                    if i == app.current_index {
                        ListItem::new(text.clone()).style(Style::default().fg(Color::Yellow))
                    } else {
                        ListItem::new(text.clone()).style(Style::default().fg(Color::White))
                    }
                })
                .collect();
            let list = List::new(list_items).block(
                Block::default()
                    .title("Opertion")
                    .borders(Borders::ALL)
                    .border_style(Style::default()),
            );
            let value_paragraph = Paragraph::new(if app.field_or_value == FieldType::Value {
                app.current_input.clone()
            } else {
                match &app.condition_value {
                    Some(v) => v.clone(),
                    None => "".to_string(),
                }
            })
            .block(
                Block::default()
                    .title("Value")
                    .borders(Borders::ALL)
                    .border_style(if app.field_or_value == FieldType::Value {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    }),
            );
            frame.render_widget(field_paragraph, chunks[0]);
            frame.render_widget(list, chunks[1]);
            frame.render_widget(value_paragraph, chunks[2]);
        }
        CurrentScreen::InputFilePath => {
            let paragraph = Paragraph::new(app.current_input.clone())
                .block(
                    Block::default()
                        .title("Enter File Path")
                        .borders(Borders::ALL),
                )
                .style(Style::default().fg(Color::White));

            frame.render_widget(paragraph, frame.area());
        }
    }
}
