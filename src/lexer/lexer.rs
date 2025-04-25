use crate::lexer::token::Token;

fn lex_num(code: &str) -> Option<Token> {
    match code.parse() {
        Ok(n) => Some(Token::Number(n)),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num() {
        let num = "1".to_string();
        let result = lex_num(&num);
        assert_eq!(result, Some(Token::Number(1)));

        let num = "11".to_string();
        let result = lex_num(&num);
        assert_eq!(result, Some(Token::Number(11)));

        let num = "k".to_string();
        let result = lex_num(&num);
        assert_eq!(result, None);
    }
}
