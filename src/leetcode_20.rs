pub fn is_valid(s: String) -> bool {
    fn is_left(c: char) -> bool {
        matches!(c, '(' | '{' | '[')
    }

    fn is_right(c: char) -> bool {
        matches!(c, ')' | '}' | ']')
    }

    fn map(c: char) -> char {
        match c {
            ')' => '(',
            '}' => '{',
            ']' => '[',
            _ => unreachable!(),
        }
    }
    let mut stack = Vec::with_capacity(s.len() / 2);
    for char in s.chars() {
        if is_left(char) {
            stack.push(char);
        } else if is_right(char) {
            if let Some(left) = stack.pop() {
                if map(char) != left {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            unreachable!()
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod test {
    use crate::leetcode_20::is_valid;

    #[test]
    fn test() {
        println!("{:?}", is_valid("()[]{}".to_string()));
        println!("{:?}", is_valid("(]".to_string()));
    }
}
