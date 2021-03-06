use super::TestMessage;
use serde;
use serde::de::Deserialize;

impl serde::Deserialize for TestMessage {
    fn deserialize<D>(deserializer: &mut D) -> Result<TestMessage, D::Error>
        where D: serde::de::Deserializer
    {
        deserializer.visit_named_map("TestMessage", TestMessageVisitor)
    }
}

enum Field {
    MESSAGE,
    VALUES,
}

impl serde::Deserialize for Field {
    fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
        where D: serde::de::Deserializer
    {
        struct FieldVisitor;

        impl serde::de::Visitor for FieldVisitor {
            type Value = Field;

            fn visit_str<E>(&mut self, value: &str) -> Result<Field, E>
                where E: serde::de::Error
            {
                match value {
                    "message" => Ok(Field::MESSAGE),
                    "values" => Ok(Field::VALUES),
                    _ => Err(serde::de::Error::syntax_error()),
                }
            }
        }

        deserializer.visit(FieldVisitor)
    }
}


struct TestMessageVisitor;

impl serde::de::Visitor for TestMessageVisitor {
    type Value = TestMessage;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<TestMessage, V::Error>
        where V: serde::de::MapVisitor
    {
        let mut message = None;
        let mut values = None;

        loop {
            match try!(visitor.visit_key()) {
                Some(Field::MESSAGE) => { message = Some(try!(visitor.visit_value())); }
                Some(Field::VALUES) => { values = Some(try!(visitor.visit_value())); }
                None => { break; }
            }
        }

        let message_final = match message {
            Some(message) => message,
            None => try!(visitor.missing_field("message")),
        };

        let values_final = match values {
            Some(values) => values,
            None => try!(visitor.missing_field("values")),
        };

        try!(visitor.end());

        Ok(TestMessage::new(message_final, values_final))
    }
}
