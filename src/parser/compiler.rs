use crate::parser::tokenizer::Tokenizer;
use crate::parser::parser::Parser;
use crate::parser::expression::Evaluate;

pub fn compile(source: String) -> String {
    let tokens = Tokenizer::new().tokenize(source);
    let expressions = Parser::new().parse(&tokens);
    let mut buffer = String::new();

    for expression in expressions {
        if buffer.len() > 0 {
            buffer.push_str("\n");
        }
        let res = expression.evaluate();
        buffer.push_str(res.as_str());
    }
    buffer
}