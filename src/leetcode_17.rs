pub fn letter_combinations(digits: String) -> Vec<String> {
    let mut r = vec![];
    if digits.is_empty() || digits.len() > 4 {
        r
    } else {
        for char in digits.chars() {
            let cu8 = char as u8 - '0' as u8;
            if cu8 >= 2 && cu8 <= 9 {
                let base = (cu8 - 2) * 3 + 'a' as u8;
                let (start, end) = match cu8 {
                    7 => (base, base + 3),
                    8 => (base + 1, base + 3),
                    9 => (base + 1, base + 4),
                    _ => (base, base + 2),
                };
                if r.is_empty() {
                    (start..=end).into_iter().for_each(|letter| r.push(String::from(char::from(letter))));
                } else {
                    r = r.into_iter().flat_map(|origin| {
                        (start..=end).into_iter().map(|letter| {
                            let mut s = String::from(&origin);
                            s.push(char::from(letter));
                            s
                        }).collect::<Vec<String>>()
                    }).collect::<Vec<String>>()
                }
            }
        }
        r
    }
}




#[cfg(test)]
mod test {
    use crate::leetcode_17::{letter_combinations};

    #[test]
    fn test() {
        println!("{:?}", letter_combinations("23".to_string()));
        println!("{:?}", letter_combinations("".to_string()));
        println!("{:?}", letter_combinations("2".to_string()));
    }
}