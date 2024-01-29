pub fn simplify_path(path: String) -> String {
    fn parse_word(word: &mut Vec<char>, stack: &mut Vec<Vec<char>>) {
        match word.len() {
            0 => {
                return;
            }
            1 => {
                if word[0] != '.' {
                    stack.push(word.clone());
                }
            }
            2 => {
                if word[0] == '.' && word[1] == '.' {
                    stack.pop();
                } else {
                    stack.push(word.clone());
                }
            }
            _ => stack.push(word.clone()),
        }
        word.clear();
    }
    let mut stack = Vec::with_capacity(path.len());
    let mut word = Vec::with_capacity(path.len());
    for char in path.chars() {
        if char == '/' {
            parse_word(&mut word, &mut stack);
        } else {
            word.push(char);
        }
    }
    parse_word(&mut word, &mut stack);
    match stack.len() {
        0 => "/".to_string(),
        len => (0..len)
            .flat_map(|i| ['/'].iter().chain(stack[i].iter()))
            .collect::<String>(),
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode_71::simplify_path;

    #[test]
    fn test() {
        println!("{:?}", simplify_path("/a/./b/../../c/".to_string()));
        println!("{:?}", simplify_path("/a/./b/../../c".to_string()));
        println!("{:?}", simplify_path("/home/".to_string()));
        println!("{:?}", simplify_path("/home//foo/".to_string()));
        println!("{:?}", simplify_path("/a//b////c/d//././/..".to_string()));
    }
}
