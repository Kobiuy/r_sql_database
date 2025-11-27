use clap::Parser;
use r_sql_database::arguments::{Args, UiType, run_command_line, run_graphic_interface};
use std::io::{self};

// fn seed(db: &mut Database<String>, history: &mut CommandHistory) {
//     let commands = [
//         r#"CREATE library KEY id FIELDS id: String, title: String, year: Int, pages: Int, rating: Float, topic: String, is_foundational: Bool"#,
//         r#"INSERT id = "lib1", title = "Homotopy Type Theory: Univalent Foundations", year = 2013, pages = 600, rating = 4.8, topic = "Foundations", is_foundational = true INTO library"#,
//         r#"INSERT id = "lib2",title = "Introduction to HoTT",year = 2018,pages = 320,rating = 4.2,topic = "Introductory",is_foundational = falseINTO library"#,
//         r#"INSERT id = "lib3", title = "Cubical Type Theory Notes", year = 2020, pages = 210, rating = 4.5, topic = "Cubical", is_foundational = false INTO library"#,
//     ];

//     for cmd in commands {
//         handle_input(cmd.trim().to_string(), db, history);
//     }
// }

fn main() -> io::Result<()> {
    let args = Args::parse();

    match args.ui_type {
        UiType::Command => run_command_line(args.key_type),
        UiType::Graphic => run_graphic_interface(args.key_type),
    }
}
