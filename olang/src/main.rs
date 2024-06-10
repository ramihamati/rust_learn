use crate::lexer::lexer::Lexer;

mod lexer;

fn main() {
    let code = "{}()";

    let mut lexer = Lexer::new(code);
    let tokens = lexer.scan_tokens();

    match tokens{
        Ok(tokens) => {
            println!("tokens {:?}", tokens)
        },
        Err(e) => {
            println!("errors: \n{}", e)
        }
    }
    println!("Hello, world!");
}
