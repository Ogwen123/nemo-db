use std::cmp::PartialEq;
use std::fmt::Formatter;

pub type TokenisedSQL = Vec<String>;

enum TokeniserErrorType {
    UnclosedString,
    UnclosedBracket
}

impl TokeniserErrorType {
    fn string(&self) -> String {
        match self {
            TokeniserErrorType::UnclosedString => String::from("Syntax Error: Unclosed String"),
            TokeniserErrorType::UnclosedBracket => String::from("Syntax Error: Unclosed Bracket")
        }
    }
}

pub struct TokeniserError {
    pub error_type: TokeniserErrorType,
    pub message: String,
}

impl TokeniserError {
    fn new<T>(_type: TokeniserErrorType, error: T) -> TokeniserError
    where T: ToString {
        TokeniserError {
            error_type: _type,
            message: error.to_string()
        }
    }
}

impl std::fmt::Debug for TokeniserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.error_type.string(), self.message)
    }
}

#[derive(PartialEq)]
struct StringTracker {
    opener: String,
    pos: usize
}

// used to track where we are when traversing the split sql
#[derive(PartialEq)]
enum PositionType {
    Normal,
    InString(StringTracker)
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

    let mut pos: PositionType = PositionType::Normal;
    let mut buffer = String::new();

    for (index, val) in split_sql.iter().enumerate() {
        if pos == PositionType::Normal {
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
                    pos = PositionType::InString(StringTracker {
                        opener: val.clone(),
                        pos: index
                    });
                    buffer += val
                } else {
                    buffer += val;
                }
            }
        } else {
            let char: String = match &pos {
                PositionType::InString(res) => res.opener.clone(),
                _ => String::new()
            };

            if val == &char {
                // check if it is escaped

                if index > 0 && split_sql[index - 1] == "\\"{
                    buffer += val
                } else {
                    buffer += val;
                    pos = PositionType::Normal;
                    processed_sql.push(buffer);
                    buffer = String::new();
                }
            } else {
                buffer += val;
            }
        }
    }

    match pos {
        PositionType::InString(res) => {
            return Err(TokeniserError::new(TokeniserErrorType::UnclosedString, format!("{}: {} {}", TokeniserErrorType::UnclosedString.string(), "Unclosed string at pos", res.pos)))
        },
        _ => {}
    }

    Ok(processed_sql)
}
