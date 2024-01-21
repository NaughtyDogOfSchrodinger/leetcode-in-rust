pub fn reverse_words(s: String) -> String {
    let mut r = String::new();
    let (mut start, mut count) = (0, 0);
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            if count == 0 {
                continue;
            } else {
                if !r.is_empty() {
                    r.insert(0, ' ');
                }
                r.insert_str(0, &s[start..start + count]);
                count = 0;
            }
        } else {
            if count == 0 {
                start = i;
            }
            count += 1;
        }
    }
    if count != 0 {
        if !r.is_empty() {
            r.insert(0, ' ');
        }
        r.insert_str(0, &s[start..start + count]);
    }
    r
}

#[cfg(test)]
mod test {
    use crate::leetcode_151::reverse_words;

    #[test]
    fn test() {
        println!("{:?}", reverse_words("luffy is still joyboy".to_string()));
    }
}