use crate::database::query::constants::Token;
use crate::database::query::parser::AST;
use crate::database::query::tokeniser::{tokenise};
use crate::utils::logger::warning;

pub fn handle_sql(_sql: String) {
    let tokens: Vec<Token> = match tokenise(_sql) {
        Ok(res) => res,
        Err(err) => {
            warning!("{}", err.message);
            return;
        }
    };
    
    let ast = match AST::parse(tokens) {
        Ok(res) => res,
        Err(err) => {
            warning!("{}", err.message);
            return;
        }
    };
}