use crate::database::query::parser::AST;
use crate::database::query::tokeniser::{tokenise, TokenisedSQL};
use crate::utils::logger::warning;

pub fn handle_sql(_sql: String) {
    let tokens: TokenisedSQL = match tokenise(_sql) {
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