
use super::*;

#[test]
fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";
    assert_eq!(search(query, contents), vec!["1: safe, fast, productive."]);
}