#[cfg(test)]
mod tests;

mod lexer;

fn main() {
    let code = "let xyz = 5;";
    let code = r#"
let five = 5;
let ten = 10;
        "#;
    let mut lexer = lexer::Lexer::new(code.to_string());

    println!("{code}");

    println!("Tokens:");

    loop {
        let token = lexer.next_token();
        if token != lexer::Tokens::INDENDATION {
            println!("{:?}", token);
        }
        if token == lexer::Tokens::EOF {
            break;
        }
    }
}
