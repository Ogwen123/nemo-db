mod database;
mod utils;

use std::io;
use std::io::Write;
use crate::utils::logger::{info};
use crate::database::sql::sql;
use crate::utils::utils::cleanup;

fn exit() {
    // add safety code here once needed
    info!("Exiting");
    std::process::exit(0);
}

fn main() {
    loop {
        let mut input: String = String::new();

        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let parsed_input = cleanup(input);
        match parsed_input.as_str() {
            "/exit" => exit(),
            _ => sql::handle_possible_sql(parsed_input)
        }
    }
}
