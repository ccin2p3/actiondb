mod literal;
mod parser;
mod node;

pub use self::node::CompiledPattern;
pub use self::node::TokenType;
pub use self::node::Node;
pub use self::literal::{LiteralNode};
pub use self::parser::ParserNode;

#[cfg(test)]
mod test {
    use matcher::trie::node::Node;
    use matcher::trie::TrieOperations;

    #[test]
    fn test_given_node_when_literals_are_inserted_in_chains_then_they_can_be_looked_up() {
        let mut node = Node::new();

        let _ = node.insert_literal("appl").insert_literal("et").insert_literal("ree");
        println!("{:?}", &node);
        assert_eq!(node.lookup_literal_mut("applet").is_ok(), true);
        assert_eq!(node.lookup_literal_mut("appletree").is_ok(), true);
        assert_eq!(node.lookup_literal_mut("appletre").is_ok(), false);
    }
}
