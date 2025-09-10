use super::super::database::query::tokeniser::splitter;

#[test]
fn test_tokeniser() {
    let target = vec!["SELECT", "*", "FROM", "table", ";"];
    let tokenised = splitter("SELECT * FROM table;").unwrap();
    println!("splitter output: {:?}", tokenised);
    assert_eq!(tokenised, target);
}

#[test]
fn test_tokeniser_string_rebuild() {
    let target = vec!["SELECT", "*", "FROM", "table", "WHERE", "id", "=", "\"this is a test\"", ";"];
    let tokenised = splitter("SELECT * FROM table WHERE id = \"this is a test\";").unwrap();
    println!("splitter output: {:?}", tokenised);
    assert_eq!(tokenised, target);
}
