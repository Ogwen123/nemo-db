use super::super::database::query::tokeniser::tokenise;

#[test]
fn test_tokeniser() {
    let target = vec!["SELECT", "*", "FROM", "table", ";"];
    let tokenised = tokenise("SELECT * FROM table;").unwrap();
    println!("{:?}", tokenised);
    assert_eq!(tokenised, target);
}

#[test]
fn test_tokeniser_string_rebuild() {
    let target = vec!["SELECT", "*", "FROM", "table", "WHERE", "id", "=", "\"this is a test\"", ";"];
    let tokenised = tokenise("SELECT * FROM table WHERE id = \"this is a test\";").unwrap();
    println!("{:?}", tokenised);
    assert_eq!(tokenised, target);
}
