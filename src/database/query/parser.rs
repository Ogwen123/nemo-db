use crate::database::query::constants::Token;
use crate::database::query::parser::ASTErrorTypes::SYNTAX;

enum ASTErrorTypes {
    SYNTAX
}

impl ASTErrorTypes {
    fn string(&self) -> String {
        match self {
            ASTErrorTypes::SYNTAX => String::from("Syntax Error"),
        }
    }
}

pub struct ASTError {
    pub error_type: ASTErrorTypes,
    pub message: String
}

impl ASTError {
    fn new(_type: ASTErrorTypes, error: &str) -> ASTError {
        ASTError {
            error_type: _type,
            message: error.to_string()
        }
    }
}

pub struct AST {

}

impl AST {
    pub fn parse(tokens: Vec<Token>) -> Result<AST, ASTError> {
        Err(ASTError::new(SYNTAX, "temp"))
    }
}