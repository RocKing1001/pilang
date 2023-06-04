use crate::lexer;

#[test]
fn check_tokens() {
    let code = "{}()=+";
    let mut lexer = lexer::Lexer::new(code.to_string());

    assert_eq!(lexer.next_token(), lexer::Tokens::LBRACE);
    assert_eq!(lexer.next_token(), lexer::Tokens::RBRACE);
    assert_eq!(lexer.next_token(), lexer::Tokens::LPAREN);
    assert_eq!(lexer.next_token(), lexer::Tokens::RPAREN);
    assert_eq!(lexer.next_token(), lexer::Tokens::ASSIGN);
    assert_eq!(lexer.next_token(), lexer::Tokens::PLUS);
    assert_eq!(lexer.next_token(), lexer::Tokens::EOF);
}
