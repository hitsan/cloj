use crate::lexer::token::Token;

fn lex_num(input: &str) -> Option<Token> {
    let nums: Vec<_> = input.chars()
        .take_while(|&num| num.is_numeric())
        .collect();
    if nums.iter().len() == 0 {
        return None;
    }
    let num: i32 = nums.iter().fold(0, |sum, n| {
        let n = n.to_digit(10).unwrap() as i32;
        sum*10+n
    });
    Some(Token::Number(num))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num() {
        let num = "1";
        let result = lex_num(&num);
        assert_eq!(result, Some(Token::Number(1)));

        let num = "11";
        let result = lex_num(&num);
        assert_eq!(result, Some(Token::Number(11)));

        let num = "k";
        let result = lex_num(&num);
        assert_eq!(result, None);

        let num = "1 1";
        let result = lex_num(&num);
        assert_eq!(result,Some(Token::Number(1)));
    }
}
