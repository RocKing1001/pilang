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

#[test]
fn check_idents() {
    let code = "let xyz = 69;";
    let mut lexer = lexer::Lexer::new(code.to_string());

    assert_eq!(lexer.next_token(), lexer::Tokens::LET);
    assert_eq!(lexer.next_token(), lexer::Tokens::INDENDATION);
    assert_eq!(lexer.next_token(), lexer::Tokens::IDENT("xyz".to_string()));
    assert_eq!(lexer.next_token(), lexer::Tokens::INDENDATION);
    assert_eq!(lexer.next_token(), lexer::Tokens::ASSIGN);
    assert_eq!(lexer.next_token(), lexer::Tokens::INDENDATION);
    assert_eq!(lexer.next_token(), lexer::Tokens::INT(69));
    assert_eq!(lexer.next_token(), lexer::Tokens::SEMICOLON);
    assert_eq!(lexer.next_token(), lexer::Tokens::EOF);
}
