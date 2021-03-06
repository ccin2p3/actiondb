extern crate actiondb;

use actiondb::matcher::Factory;
use actiondb::matcher::matcher::builder::BuildError;

#[test]
fn test_given_pattern_file_when_its_syntax_is_ok_then_matcher_can_be_built_from_it() {
    let pattern_file_path = "tests/matcher/ssh_ok.pattern";
    let matcher = Factory::from_plain_file(pattern_file_path);
    assert_eq!(matcher.is_ok(), true);
}

#[test]
fn test_given_pattern_file_when_its_syntax_is_not_ok_then_matcher_cannot_be_built_from_it() {
    let pattern_file_path = "tests/matcher/ssh_wrong.pattern";
    match Factory::from_plain_file(pattern_file_path) {
        Err(BuildError::FromPlain(_)) => {},
        _ => unreachable!()
    }
}

#[test]
fn test_given_json_file_when_its_syntax_is_ok_then_matcher_can_be_built_from_it() {
    let pattern_file_path = "tests/matcher/ssh_ok.json";
    let matcher = Factory::from_json_file(pattern_file_path);
    println!("{:?}", &matcher);
    matcher.ok().expect("Failed to create a Matched from a valid JSON pattern file");
}

#[test]
fn test_given_json_file_when_its_syntax_is_not_ok_then_matcher_cannot_be_built_from_it() {
    let pattern_file_path = "tests/matcher/ssh_wrong.json";
    let matcher = Factory::from_json_file(pattern_file_path);
    matcher.err().expect("Failed to get an error when a Matcher is created from an invalid JSON file");
}

#[test]
fn test_given_non_existing_json_file_when_it_is_loaded_then_matcher_cannot_be_created_from_it() {
    let pattern_file_path = "tests/matcher/ssh_non_existing.json";
    let matcher = Factory::from_json_file(pattern_file_path);
    matcher.err().expect("Failed to get an error when a Matcher is created from a non-existing JSON file");
}

#[test]
fn test_given_json_file_when_matcher_is_created_by_factory_then_the_right_file_type_is_used_based_on_the_extension() {
    let pattern_file_path = "tests/matcher/ssh_ok.json";
    let matcher = Factory::from_file(pattern_file_path);
    println!("{:?}", &matcher);
    matcher.ok().expect("Failed to create a Matcher from a valid JSON pattern file");
}

#[test]
fn test_given_plain_file_when_matcher_is_created_by_factory_then_the_right_file_type_is_used_based_on_the_extension() {
    let pattern_file_path = "tests/matcher/ssh_ok.pattern";
    let matcher = Factory::from_file(pattern_file_path);
    println!("{:?}", &matcher);
    matcher.ok().expect("Failed to create a Matcher from a valid JSON pattern file");
}
