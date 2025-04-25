fn lex_num(code: &str) -> i32 {
    let num: i32 = code.parse().unwrap();
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num() {
        let num = "1".to_string();
        let result = lex_num(&num);
        assert_eq!(result, 1);
    }
}
