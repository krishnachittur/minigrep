
use super::*;

#[test]
fn simple_test() {
    let query = "dog";
    let contents = "\
The quick brown fox
jumped over the lazy dog
and the dog wasn't pleased.";
    assert_eq!(search(query, contents),
    vec!["1: jumped over the lazy dog",
        "2: and the dog wasn't pleased."]
    );
}