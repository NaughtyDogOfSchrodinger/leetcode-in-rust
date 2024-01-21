pub fn length_of_last_word(s: String) -> i32 {
    let mut count = 0;
    let mut r = 0;
    for c in s.chars() {
        if c == ' ' {
            if count == 0 {
                continue
            } else {
                r = count;
                count = 0;
            }
        } else {
            count += 1;
        }
    }
    if count != 0 {
        count
    } else {
        r
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode_58::length_of_last_word;

    #[test]
    fn test() {
        println!("{:?}", length_of_last_word("luffy is still joyboy".to_string()));
    }
}