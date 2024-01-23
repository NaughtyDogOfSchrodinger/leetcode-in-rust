
pub fn word_pattern(pattern: String, s: String) -> bool {
    use std::collections::HashMap;

    let mut words = s.split_ascii_whitespace();
    let mut chars = pattern.chars();
    if words.clone().count() != pattern.len() {
        return false;
    }
    let mut map = HashMap::<String, String>::new();
    while let (Some(char), Some(word)) = (chars.next(), words.next()) {
        let c_key = String::from(char);
        let mut w_key = String::from(word);
        w_key.push(' ');
        match (map.get(&c_key), map.get(&w_key)) {
            (Some(wv), Some(cv)) => {
                if !wv.eq(word) || !cv.eq(&c_key){
                    return false
                }
            },
            (None, None) => {
                map.insert(c_key.clone(), String::from(word));
                map.insert(w_key, c_key);
            }
            _ => return false,
        }
    }
    true
}


#[cfg(test)]
mod test {
    use crate::leetcode_290::word_pattern;

    #[test]
    fn test() {
        // println!("{:?}", is_isomorphic("egg".to_string(), "add".to_string()));
        // println!("{:?}", is_isomorphic("foo".to_string(), "bar".to_string()));
        // println!("{:?}", is_isomorphic("badc".to_string(), "baba".to_string()));
        println!("{:?}", word_pattern("abba".to_string(), "dog cat cat dog".to_string()));
        println!("{:?}", word_pattern("aaaa".to_string(), "dog cat cat dog".to_string()));
        println!("{:?}", word_pattern("abba".to_string(), "dog cat cat fish".to_string()));
    }
}