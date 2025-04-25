use crate::lexer::token;

fn lex_num(code: &str) -> Option<i32> {
    match code.parse() {
        Ok(n) => Some(n),
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
        assert_eq!(result, Some(1));

        let num = "11".to_string();
        let result = lex_num(&num);
        assert_eq!(result, Some(11));

        let num = "k".to_string();
        let result = lex_num(&num);
        assert_eq!(result, None);

    }
}
