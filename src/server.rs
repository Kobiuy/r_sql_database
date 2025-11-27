use clap::Parser;
use r_sql_database::{
    arguments::Args, command_history::CommandHistory, database::AnyDatabase,
    handlers::handle_input_any_db,
};

fn handle_client(
    buffer: &[u8],
    bytes_received: usize,
    db: &mut AnyDatabase,
    history: &mut CommandHistory,
) -> Vec<u8> {
    let input_raw = &buffer[..bytes_received];

    let input_str = match String::from_utf8(input_raw.to_vec()) {
        Ok(s) => s.trim().to_string(),
        Err(e) => {
            return format!("Bad input : {}\n", e).as_bytes().to_vec();
        }
    };

    let result = match handle_input_any_db(input_str, db, history) {
        Ok(r) => r.to_string(),
        Err(e) => e.to_string(),
    };

    result.into_bytes()
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let socket = std::net::UdpSocket::bind("127.0.0.1:7878")?;
    let mut buffer = [0u8; 512];

    let mut db: AnyDatabase = args.key_type.to_database();
    let mut history = CommandHistory::new();

    println!("UDP server running on 127.0.0.1:7878...");

    loop {
        let (bytes_received, src_addr) = socket.recv_from(&mut buffer)?;

        let response = handle_client(&buffer, bytes_received, &mut db, &mut history);

        socket.send_to(&response, src_addr)?;
    }
}
