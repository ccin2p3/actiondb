use std::hash::{SipHasher, Hash, Hasher};

use parsers::{Parser, ObjectSafeHash, SetParser, HasOptionalParameter, OptionalParameter};

#[derive(Clone, Debug, Hash)]
pub struct IntParser {
    delegate: SetParser
}

impl IntParser {
    pub fn from_str(name: &str) -> IntParser {
        IntParser::new(name.to_string())
    }

    pub fn new(name: String) -> IntParser {
        let delegate = SetParser::new(name, "0123456789");
        IntParser{ delegate: delegate }
    }

    pub fn set_min_length(&mut self, length: usize) {
        self.delegate.set_min_length(length)
    }

    pub fn set_max_length(&mut self, length: usize) {
        self.delegate.set_max_length(length)
    }
}

impl Parser for IntParser {
    fn parse<'a, 'b>(&'a self, value: &'b str) -> Option<(&'a str, &'b str)> {
        self.delegate.parse(value)
    }

    fn name(&self) -> &str {
        self.delegate.name()
    }

    fn boxed_clone(&self) -> Box<Parser> {
        Box::new(self.clone())
    }
}

impl ObjectSafeHash for IntParser {
    fn hash_os(&self) -> u64 {
        let mut hasher = SipHasher::new();
        "parser:int".hash(&mut hasher);
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl HasOptionalParameter for IntParser {
    fn set_optional_params<'a>(&mut self, params: &Vec<OptionalParameter<'a>>) -> bool {
        self.delegate.set_optional_params(params)
    }
}

#[cfg(test)]
mod test {
    use parsers::{IntParser, Parser};

    #[test]
    fn test_given_int_parser_when_the_match_is_empty_then_the_result_isnt_successful() {
        let parser = IntParser::from_str("test_int_parser");
        assert_eq!(parser.parse(""), None);
        assert_eq!(parser.parse("asdf"), None);
    }

    #[test]
    fn test_given_matching_string_when_it_is_parsed_then_it_matches() {
        let parser_name = "test_int_parser";
        let parser = IntParser::from_str(parser_name);
        assert_eq!(parser.parse("1234asd").unwrap(), (parser_name, "1234"));
    }

    #[test]
    fn test_given_matching_string_which_is_longer_than_the_max_match_length_when_it_is_parsed_then_it_does_not_match() {
        let parser_name = "test_int_parser";
        let mut parser = IntParser::from_str(parser_name);
        parser.set_max_length(3);
        assert_eq!(parser.parse("1234asd"), None);
    }
}
