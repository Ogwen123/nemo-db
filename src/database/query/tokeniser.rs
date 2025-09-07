use std::cmp::PartialEq;
use std::default::Default;
use std::fmt::Formatter;
use crate::database::query::tokeniser::PositionType::Normal;

enum TokensTypes {
    Keyword(String),
    Identifier(String),
    Punctuation(String),
    Value(String)
}

type TokenisedSQL = Vec<String>;
enum TokeniserErrorType {
    KEYWORD,
    OPERATOR
}

impl TokeniserErrorType {
    fn string(&self) -> String {
        match self {
            TokeniserErrorType::KEYWORD => String::from("Keyword Error"),
            TokeniserErrorType::OPERATOR => String::from("Operator Error")
        }
    }
}

impl std::fmt::Debug for TokeniserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.error_type.string(), self.message)
    }
}

pub struct TokeniserError {
    error_type: TokeniserErrorType,
    message: String,
}

// used to track where we are when traversing the split sql
#[derive(PartialEq)]
enum PositionType {
    Normal,
    String(String)
}


pub fn tokenise<T>(_sql: T) -> Result<TokenisedSQL, TokeniserError>
where T: ToString {
    let sql = _sql.to_string();

    let split_sql: Vec<String> = sql
        .split("")
        .map(|x|
            x.to_string()
        )
        .collect::<Vec<String>>();

    let mut processed_sql: Vec<String> = Vec::new();

    let mut pos: PositionType = Normal;
    let mut buffer = String::new();

    for (index, val) in split_sql.iter().enumerate() {
        if pos == Normal {
            if val == ";" {
                if buffer.len() > 0 {processed_sql.push(buffer)}
                processed_sql.push(";".to_string());
                break
            }

            if val == " " {
                processed_sql.push(buffer);
                buffer = String::new()
            } else {
                if val == "\"" || val == "'" {
                    pos = PositionType::String(val.clone());
                    buffer += val
                } else {
                    buffer += val;
                }
            }
        } else {
            let char: String = match &pos {
                PositionType::String(res) => res.clone(),
                _ => String::new()
            };

            if val == &char {
                // check if it is escaped

                if index > 0 && split_sql[index - 1] == "\\"{
                    buffer += val
                } else {
                    buffer += val;
                    pos = Normal;
                    processed_sql.push(buffer);
                    buffer = String::new();
                }
            } else {
                buffer += val;
            }
        }
    }

    Ok(processed_sql)
}
