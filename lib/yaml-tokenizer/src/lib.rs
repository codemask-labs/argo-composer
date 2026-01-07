#[derive(Debug)]
pub enum TokenKind {
    Indent,
    Array,
    Object,
    String, // quoted strings
    Scalar, // bare words, unquoted scalars
    Number,
    Boolean,
    Null,
    LF,
    CR,
}

#[derive(Debug)]
pub struct YamlNode {
    kind: TokenKind,
    span: Option<Vec<YamlNode>>,
}

pub struct YamlTokenizer {}

impl YamlTokenizer {
    pub fn to_tokens(contents: String) -> Vec<YamlNode> {
        let result = Vec::new();

        for char in contents.chars() {
            println!("char :: {}", char);
        }

        result
    }
}

#[cfg(test)]
mod unit_test {
    use crate::YamlTokenizer;

    const document: &str = "
    example:
        hello: world
        isTrue: true
        isFalse: false
        array: [test, thing, 1]
        arrayOfObjects:
            - name: test
              isFirst: true
    ";

    #[test]
    fn get_tokens_from_a_example() {
        let tokens = YamlTokenizer::to_tokens(document.to_string());

        println!("tokens :: {:?}", tokens);
        assert!(false);
    }
}
