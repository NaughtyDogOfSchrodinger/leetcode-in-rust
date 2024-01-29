pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    fn token2num(token: &String) -> i32 {
        let mut sum = 0;
        let mut multi = 1;
        for (i, c) in token.chars().enumerate() {
            if i == 0 && matches!(c, '-' | '+') {
                if c == '-' {
                    multi = -1;
                }
                continue;
            }
            let num = c as u8 - '0' as u8;
            assert!(num <= 9);
            sum = sum * 10 + num as i32 * multi;
        }
        sum
    }
    match tokens.len() {
        1 => token2num(&tokens[0]),
        len => {
            let mut stack = Vec::with_capacity(len);
            for token in tokens {
                if matches!(token.as_str(), "+" | "-" | "*" | "/") {
                    if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                        match token.as_str() {
                            "+" => stack.push(a + b),
                            "-" => stack.push(a - b),
                            "*" => stack.push(a * b),
                            "/" => stack.push(a / b),
                            _ => unreachable!(),
                        }
                    } else {
                        unreachable!()
                    }
                } else {
                    stack.push(token2num(&token));
                }
            }
            stack.pop().unwrap_or(0)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode_150::eval_rpn;

    #[test]
    fn test() {
        println!(
            "{:?}",
            eval_rpn(vec![
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "3".to_string(),
                "*".to_string()
            ])
        );
        println!(
            "{:?}",
            eval_rpn(vec![
                "4".to_string(),
                "13".to_string(),
                "5".to_string(),
                "/".to_string(),
                "+".to_string()
            ])
        );
        println!(
            "{:?}",
            eval_rpn(vec![
                "10".to_string(),
                "6".to_string(),
                "9".to_string(),
                "3".to_string(),
                "+".to_string(),
                "-11".to_string(),
                "*".to_string(),
                "/".to_string(),
                "*".to_string(),
                "17".to_string(),
                "+".to_string(),
                "5".to_string(),
                "+".to_string()
            ])
        );
    }
}
